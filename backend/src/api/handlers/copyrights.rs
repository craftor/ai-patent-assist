use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::handlers::ApiResponse;
use crate::AppState;
use crate::models::AiModelConfig;

/// 软著文档数据结构
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CopyrightDocument {
    pub id: Uuid,
    pub project_id: Uuid,
    pub software_name: String,
    pub software_version: Option<String>,
    pub developer: Option<String>,
    pub completion_date: Option<chrono::NaiveDate>,
    pub publication_date: Option<chrono::NaiveDate>,
    pub software_category: Option<String>,
    pub operating_system: Option<String>,
    pub programming_language: Option<String>,
    pub line_count: Option<i32>,
    pub source_code_path: Option<String>,
    pub user_manual_path: Option<String>,
    pub description: Option<String>,
    pub function_features: Option<String>,
    pub technical_features: Option<String>,
    pub ai_prompt: Option<String>,
    pub ai_model: Option<String>,
    pub version: i32,
    pub status: String,
    pub review_comments: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

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

/// 获取软著列表
pub async fn list_copyrights(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<CopyrightDocument>>> {
    let copyrights = sqlx::query_as::<_, CopyrightDocument>(
        r#"SELECT
            id,
            project_id,
            software_name,
            software_version,
            developer,
            completion_date,
            publication_date,
            software_category,
            operating_system,
            programming_language,
            line_count,
            source_code_path,
            user_manual_path,
            description,
            function_features,
            technical_features,
            ai_prompt,
            ai_model,
            version,
            status::text as status,
            review_comments,
            reviewed_by,
            reviewed_at,
            created_at,
            updated_at
        FROM copyright_documents ORDER BY updated_at DESC"#
    )
    .fetch_all(&*state.pool)
    .await;

    match copyrights {
        Ok(copyrights) => Json(ApiResponse::success(copyrights)),
        Err(e) => {
            tracing::error!("Failed to list copyrights: {}", e);
            Json(ApiResponse::error("Failed to list copyrights"))
        }
    }
}

/// 获取单个软著详情
pub async fn get_copyright(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<CopyrightDocument>>, StatusCode> {
    let copyright = sqlx::query_as::<_, CopyrightDocument>(
        r#"SELECT
            id,
            project_id,
            software_name,
            software_version,
            developer,
            completion_date,
            publication_date,
            software_category,
            operating_system,
            programming_language,
            line_count,
            source_code_path,
            user_manual_path,
            description,
            function_features,
            technical_features,
            ai_prompt,
            ai_model,
            version,
            status::text as status,
            review_comments,
            reviewed_by,
            reviewed_at,
            created_at,
            updated_at
        FROM copyright_documents WHERE id = $1"#
    )
    .bind(id)
    .fetch_optional(&*state.pool)
    .await;

    match copyright {
        Ok(Some(copyright)) => Ok(Json(ApiResponse::success(copyright))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get copyright: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 生成软著
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
