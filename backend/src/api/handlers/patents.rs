use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::api::handlers::ApiResponse;
use crate::AppState;
use crate::models::AiModelConfig;

#[derive(Debug, Deserialize)]
pub struct GeneratePatentRequest {
    pub title: String,
    pub technical_field: String,
    pub background_art: String,
    pub invention_description: String,
    pub embodiments: Option<Vec<String>>,
    pub claims_input: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatentResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub duration_ms: i32,
}

pub async fn generate_patent(
    State(state): State<AppState>,
    Json(payload): Json<GeneratePatentRequest>,
) -> Json<ApiResponse<PatentResponse>> {
    match state.ai_generator.generate_patent(
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
        &payload.title,
        &payload.technical_field,
        &payload.background_art,
        &payload.invention_description,
        &payload.embodiments.unwrap_or_default(),
        payload.claims_input.as_deref(),
    ).await {
        Ok(result) => Json(ApiResponse::success(PatentResponse {
            id: uuid::Uuid::new_v4().to_string(),
            title: payload.title,
            content: result.content,
            input_tokens: result.input_tokens,
            output_tokens: result.output_tokens,
            duration_ms: result.duration_ms,
        })),
        Err(e) => Json(ApiResponse::error(format!("生成失败：{}", e))),
    }
}
