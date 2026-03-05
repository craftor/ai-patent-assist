mod auth;
mod projects;
mod patents;
mod copyrights;

pub use auth::{login, register, get_test_account};
pub use projects::list_projects;
pub use patents::generate_patent;
pub use copyrights::generate_copyright;

use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

/// 通用 API 响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            data: None,
        }
    }
}

/// 健康检查
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
