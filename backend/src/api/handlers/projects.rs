use axum::Json;
use serde::{Deserialize, Serialize};

use crate::api::handlers::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub project_type: String,
    pub status: String,
}

pub async fn list_projects() -> Json<ApiResponse<Vec<Project>>> {
    Json(ApiResponse::success(vec![]))
}
