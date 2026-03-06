use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::api::handlers::ApiResponse;
use crate::AppState;
use crate::models::AiModelConfig;

#[derive(Debug, Deserialize)]
pub struct GenerateCopyrightRequest {
    pub software_name: String,
    pub software_version: Option<String>,
    pub developer: String,
    pub description: String,
    pub function_features: String,
    pub technical_features: String,
    pub source_code_summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyrightResponse {
    pub id: String,
    pub software_name: String,
    pub content: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub duration_ms: i32,
}

pub async fn generate_copyright(
    State(state): State<AppState>,
    Json(payload): Json<GenerateCopyrightRequest>,
) -> Json<ApiResponse<CopyrightResponse>> {
    match state.ai_generator.generate_copyright(
        &AiModelConfig {
            id: uuid::Uuid::nil(),
            provider: "anthropic".to_string(),
            model_name: "claude-3-5-sonnet-20241022".to_string(),
            api_key_encrypted: None,
            api_endpoint: None,
            max_tokens: 4096,
            temperature: 0.7,
            is_active: true,
            priority: 1,
            metadata: None,
        },
        &payload.software_name,
        payload.software_version.as_deref(),
        &payload.developer,
        &payload.description,
        &payload.function_features,
        &payload.technical_features,
        payload.source_code_summary.as_deref(),
    ).await {
        Ok(result) => Json(ApiResponse::success(CopyrightResponse {
            id: uuid::Uuid::new_v4().to_string(),
            software_name: payload.software_name,
            content: result.content,
            input_tokens: result.input_tokens,
            output_tokens: result.output_tokens,
            duration_ms: result.duration_ms,
        })),
        Err(e) => Json(ApiResponse::error(format!("生成失败：{}", e))),
    }
}
