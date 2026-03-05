use axum::{extract::{State, Path}, http::StatusCode, Json};
use uuid::Uuid;

use crate::api::handlers::ApiResponse;

/// 列出 AI 模型
pub async fn list_models(
    State(state): State<crate::AppState>,
) -> Result<Json<ApiResponse<Vec<ModelResponse>>>, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 添加模型
pub async fn add_model(
    State(state): State<crate::AppState>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 更新模型
pub async fn update_model(
    State(state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 删除模型
pub async fn delete_model(
    State(state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 获取默认模型
pub async fn get_default_model(
    State(state): State<crate::AppState>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 设置默认模型
pub async fn set_default_model(
    State(state): State<crate::AppState>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[derive(Debug, serde::Serialize)]
pub struct ModelResponse {
    pub id: Uuid,
    pub provider: String,
    pub model_name: String,
}
