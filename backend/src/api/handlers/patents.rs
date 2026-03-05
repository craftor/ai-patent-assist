use axum::Json;
use serde::{Deserialize, Serialize};

use crate::api::handlers::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct GeneratePatentRequest {
    pub title: String,
    pub technical_field: String,
    pub background_art: String,
    pub invention_description: String,
}

pub async fn generate_patent(
    Json(payload): Json<GeneratePatentRequest>,
) -> Json<ApiResponse<PatentResponse>> {
    // 简化版本：返回占位响应
    Json(ApiResponse::success(PatentResponse {
        id: "1".to_string(),
        title: payload.title,
        content: "专利说明书生成中...".to_string(),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatentResponse {
    pub id: String,
    pub title: String,
    pub content: String,
}
