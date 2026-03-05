use axum::{extract::{State, Path}, http::StatusCode, Json};
use uuid::Uuid;

use crate::api::handlers::ApiResponse;

/// 列出模板
pub async fn list_templates(
    State(state): State<crate::AppState>,
) -> Result<Json<ApiResponse<Vec<TemplateResponse>>>, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 创建模板
pub async fn create_template(
    State(state): State<crate::AppState>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 更新模板
pub async fn update_template(
    State(state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// 删除模板
pub async fn delete_template(
    State(state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // TODO: 实现
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[derive(Debug, serde::Serialize)]
pub struct TemplateResponse {
    pub id: Uuid,
    pub name: String,
    pub template_type: String,
}
