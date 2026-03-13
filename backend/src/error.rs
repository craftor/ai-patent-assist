// 统一错误处理模块

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 通用错误响应格式
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            error: ErrorBody {
                code: code.into(),
                message: message.into(),
                details: None,
            },
        }
    }

    pub fn with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            success: false,
            error: ErrorBody {
                code: code.into(),
                message: message.into(),
                details: Some(details),
            },
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new("NOT_FOUND", message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new("UNAUTHORIZED", message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new("BAD_REQUEST", message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new("INTERNAL_ERROR", message)
    }

    pub fn validation_error(message: impl Into<String>, details: serde_json::Value) -> Self {
        Self::with_details("VALIDATION_ERROR", message, details)
    }
}

/// 应用错误类型
#[derive(Debug, Error)]
pub enum AppError {
    #[error("资源不存在：{0}")]
    NotFound(String),

    #[error("认证失败：{0}")]
    Unauthorized(String),

    #[error("权限不足：{0}")]
    Forbidden(String),

    #[error("请求无效：{0}")]
    BadRequest(String),

    #[error("验证失败：{0}")]
    ValidationError(String),

    #[error("验证失败详情：{0}")]
    ValidationErrorWithDetails(String, serde_json::Value),

    #[error("数据库错误：{0}")]
    DatabaseError(String),

    #[error("内部错误：{0}")]
    InternalError(String),

    #[error("AI 服务错误：{0}")]
    AiServiceError(String),

    #[error("文件错误：{0}")]
    FileError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_response) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, ErrorResponse::not_found(msg)),
            AppError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, ErrorResponse::unauthorized(msg))
            }
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, ErrorResponse::new("FORBIDDEN", msg)),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, ErrorResponse::bad_request(msg)),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse::new("VALIDATION_ERROR", msg),
            ),
            AppError::ValidationErrorWithDetails(msg, details) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse::validation_error(msg, details.clone()),
            ),
            AppError::DatabaseError(msg) => {
                tracing::error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse::internal("数据库错误"))
            }
            AppError::AiServiceError(msg) => {
                tracing::error!("AI service error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new("AI_SERVICE_ERROR", msg),
                )
            }
            AppError::FileError(msg) => {
                tracing::error!("File error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse::new("FILE_ERROR", msg))
            }
            AppError::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse::internal(msg))
            }
        };

        (status, Json(error_response)).into_response()
    }
}

/// 从 SQLx 错误转换为应用错误
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("记录不存在".to_string()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

/// 从 serde_json 错误转换为应用错误
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::BadRequest(format!("JSON 解析错误：{}", err))
    }
}

/// 从 UUID 解析错误转换为应用错误
impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::BadRequest(format!("无效的 UUID: {}", err))
    }
}

/// 工具函数：将结果转换为 AppError
pub type Result<T> = std::result::Result<T, AppError>;

/// 工具函数：创建验证错误
pub fn validation_error(field: &str, message: &str) -> AppError {
    let details = serde_json::json!({
        "field": field,
        "message": message
    });
    AppError::ValidationErrorWithDetails(format!("字段 '{}' 验证失败：{}", field, message), details)
}

/// 工具函数：创建批量验证错误
pub fn validation_errors(errors: Vec<(&str, &str)>) -> AppError {
    let details = serde_json::json!({
        "errors": errors.iter().map(|(field, msg)| {
            serde_json::json!({
                "field": field,
                "message": msg
            })
        }).collect::<Vec<_>>()
    });
    AppError::ValidationErrorWithDetails("多个字段验证失败".to_string(), details)
}
