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

#[derive(Debug, Deserialize)]
pub struct UpdateCopyrightRequest {
    pub software_name: Option<String>,
    pub software_version: Option<String>,
    pub developer: Option<String>,
    pub completion_date: Option<String>,
    pub software_category: Option<String>,
    pub operating_system: Option<String>,
    pub programming_language: Option<String>,
    pub line_count: Option<i32>,
    pub source_code_path: Option<String>,
    pub user_manual_path: Option<String>,
    pub description: Option<String>,
    pub function_features: Option<String>,
    pub technical_features: Option<String>,
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

/// 更新软著
pub async fn update_copyright(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCopyrightRequest>,
) -> Result<Json<ApiResponse<CopyrightDocument>>, StatusCode> {
    let updated = sqlx::query_as::<_, CopyrightDocument>(
        r#"UPDATE copyright_documents SET
            software_name = COALESCE($2, software_name),
            software_version = COALESCE($3, software_version),
            developer = COALESCE($4, developer),
            completion_date = CASE WHEN $5 IS NOT NULL THEN $5::date ELSE completion_date END,
            software_category = COALESCE($6, software_category),
            operating_system = COALESCE($7, operating_system),
            programming_language = COALESCE($8, programming_language),
            line_count = COALESCE($9, line_count),
            source_code_path = COALESCE($10, source_code_path),
            user_manual_path = COALESCE($11, user_manual_path),
            description = COALESCE($12, description),
            function_features = COALESCE($13, function_features),
            technical_features = COALESCE($14, technical_features),
            version = version + 1,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
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
            updated_at"#
    )
    .bind(id)
    .bind(payload.software_name)
    .bind(payload.software_version)
    .bind(payload.developer)
    .bind(payload.completion_date)
    .bind(payload.software_category)
    .bind(payload.operating_system)
    .bind(payload.programming_language)
    .bind(payload.line_count)
    .bind(payload.source_code_path)
    .bind(payload.user_manual_path)
    .bind(payload.description)
    .bind(payload.function_features)
    .bind(payload.technical_features)
    .fetch_optional(&*state.pool)
    .await;

    match updated {
        Ok(Some(copyright)) => Ok(Json(ApiResponse::success(copyright))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update copyright: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
