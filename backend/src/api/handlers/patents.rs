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

/// 专利文档数据结构
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PatentDocument {
    pub id: Uuid,
    pub project_id: Uuid,
    pub patent_type: String,
    pub title: String,
    pub technical_field: Option<String>,
    pub background_art: Option<String>,
    pub invention_content: Option<String>,
    pub claims: serde_json::Value,
    pub abstract_text: Option<String>,
    pub drawings_description: Option<String>,
    pub embodiment: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct UpdatePatentRequest {
    pub title: Option<String>,
    pub technical_field: Option<String>,
    pub background_art: Option<String>,
    pub invention_content: Option<String>,
    pub claims: Option<serde_json::Value>,
    pub abstract_text: Option<String>,
    pub drawings_description: Option<String>,
    pub embodiment: Option<String>,
}

/// 获取专利列表
pub async fn list_patents(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<PatentDocument>>> {
    let patents = sqlx::query_as::<_, PatentDocument>(
        r#"SELECT
            id,
            project_id,
            patent_type::text as patent_type,
            title,
            technical_field,
            background_art,
            invention_content,
            claims,
            abstract_text,
            drawings_description,
            embodiment,
            ai_prompt,
            ai_model,
            version,
            status::text as status,
            review_comments,
            reviewed_by,
            reviewed_at,
            created_at,
            updated_at
        FROM patent_documents ORDER BY updated_at DESC"#
    )
    .fetch_all(&*state.pool)
    .await;

    match patents {
        Ok(patents) => Json(ApiResponse::success(patents)),
        Err(e) => {
            tracing::error!("Failed to list patents: {}", e);
            Json(ApiResponse::error("Failed to list patents"))
        }
    }
}

/// 获取单个专利详情
pub async fn get_patent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PatentDocument>>, StatusCode> {
    let patent = sqlx::query_as::<_, PatentDocument>(
        r#"SELECT
            id,
            project_id,
            patent_type::text as patent_type,
            title,
            technical_field,
            background_art,
            invention_content,
            claims,
            abstract_text,
            drawings_description,
            embodiment,
            ai_prompt,
            ai_model,
            version,
            status::text as status,
            review_comments,
            reviewed_by,
            reviewed_at,
            created_at,
            updated_at
        FROM patent_documents WHERE id = $1"#
    )
    .bind(id)
    .fetch_optional(&*state.pool)
    .await;

    match patent {
        Ok(Some(patent)) => Ok(Json(ApiResponse::success(patent))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get patent: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 生成专利
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

/// 更新专利
pub async fn update_patent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePatentRequest>,
) -> Result<Json<ApiResponse<PatentDocument>>, StatusCode> {
    // 更新字段
    let updated = sqlx::query_as::<_, PatentDocument>(
        r#"UPDATE patent_documents SET
            title = COALESCE($2, title),
            technical_field = COALESCE($3, technical_field),
            background_art = COALESCE($4, background_art),
            invention_content = COALESCE($5, invention_content),
            claims = COALESCE($6, claims),
            abstract_text = COALESCE($7, abstract_text),
            drawings_description = COALESCE($8, drawings_description),
            embodiment = COALESCE($9, embodiment),
            version = version + 1,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            project_id,
            patent_type::text as patent_type,
            title,
            technical_field,
            background_art,
            invention_content,
            claims,
            abstract_text,
            drawings_description,
            embodiment,
            ai_prompt,
            ai_model,
            version,
            status::text as status,
            review_comments,
            reviewed_by,
            reviewed_at,
            created_at,
            updated_at"#
    )
    .bind(id)
    .bind(payload.title)
    .bind(payload.technical_field)
    .bind(payload.background_art)
    .bind(payload.invention_content)
    .bind(payload.claims.clone())
    .bind(payload.abstract_text)
    .bind(payload.drawings_description)
    .bind(payload.embodiment)
    .fetch_optional(&*state.pool)
    .await;

    match updated {
        Ok(Some(patent)) => Ok(Json(ApiResponse::success(patent))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update patent: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
