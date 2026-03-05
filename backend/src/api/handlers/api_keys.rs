use axum::{extract::{State, Path}, http::StatusCode, Json};
use uuid::Uuid;

use crate::api::handlers::ApiResponse;

/// 列出 API Keys
pub async fn list_keys(
    State(state): State<crate::AppState>,
) -> Result<Json<ApiResponse<Vec<ApiKeyResponse>>>, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 创建 API Key
pub async fn create_key(
    State(state): State<crate::AppState>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 撤销 API Key
pub async fn revoke_key(
    State(state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[derive(Debug, serde::Serialize)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
