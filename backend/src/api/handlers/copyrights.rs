use axum::Json;
use serde::{Deserialize, Serialize};

use crate::api::handlers::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct GenerateCopyrightRequest {
    pub software_name: String,
    pub developer: String,
    pub description: String,
}

pub async fn generate_copyright(
    Json(payload): Json<GenerateCopyrightRequest>,
) -> Json<ApiResponse<CopyrightResponse>> {
    // 简化版本：返回占位响应
    Json(ApiResponse::success(CopyrightResponse {
        id: "1".to_string(),
        software_name: payload.software_name,
        content: "软著说明书生成中...".to_string(),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyrightResponse {
    pub id: String,
    pub software_name: String,
    pub content: String,
}
