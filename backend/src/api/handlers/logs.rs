use axum::{extract::State, http::StatusCode, Json};

use crate::api::handlers::ApiResponse;

/// 列出审计日志
pub async fn list_audit_logs(
    _state: State<crate::AppState>,
) -> Result<Json<ApiResponse<Vec<AuditLogResponse>>>, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 列出 AI 使用日志
pub async fn list_ai_usage(
    _state: State<crate::AppState>,
) -> Result<Json<ApiResponse<Vec<AiUsageResponse>>>, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[derive(Debug, serde::Serialize)]
pub struct AuditLogResponse {
    pub id: uuid::Uuid,
    pub action: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize)]
pub struct AiUsageResponse {
    pub id: uuid::Uuid,
    pub total_tokens: i32,
    pub status: String,
}
