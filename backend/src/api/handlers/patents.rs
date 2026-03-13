use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::handlers::ApiResponse;
use crate::models::AiModelConfig;
use crate::AppState;

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
    pub project_id: String,
    pub patent_type: String,
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
    pub project_id: String,
    pub title: String,
    pub patent_type: String,
    pub content: String,
    pub technical_field: String,
    pub background_art: String,
    pub invention_content: String,
    pub claims_text: String,
    pub abstract_text: String,
    pub status: String,
    pub version: i32,
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
pub async fn list_patents(State(state): State<AppState>) -> Json<ApiResponse<Vec<PatentDocument>>> {
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
        FROM patent_documents ORDER BY updated_at DESC"#,
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
        FROM patent_documents WHERE id = $1"#,
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

/// 生成专利并保存到数据库
pub async fn generate_patent(
    State(state): State<AppState>,
    Json(payload): Json<GeneratePatentRequest>,
) -> Json<ApiResponse<PatentResponse>> {
    // 注意：当前实现中用户 ID 从环境变量获取，实际项目中应通过 JWT token 提取
    // 这里使用默认用户 ID 用于开发环境
    let user_id = "00000000-0000-0000-0000-000000000001".to_string();

    // 解析 project_id
    let project_id = match Uuid::parse_str(&payload.project_id) {
        Ok(id) => id,
        Err(_) => {
            return Json(ApiResponse::error("无效的项目 ID"));
        }
    };

    // 调用 AI 生成服务
    let ai_result = state
        .ai_generator
        .generate_patent(
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
            &payload.embodiments.clone().unwrap_or_default(),
            payload.claims_input.as_deref(),
        )
        .await;

    match ai_result {
        Ok(result) => {
            // 解析 AI 生成的内容
            let content = &result.content;

            // 从 AI 生成的内容中提取各个部分
            // 这里使用简单的字符串查找，实际项目中可以使用更复杂的解析逻辑
            let invention_content = extract_section(content, "发明内容", Some("附图说明"))
                .unwrap_or_else(|| payload.invention_description.clone());

            let claims_text = extract_section(content, "权利要求", Some("摘要"))
                .unwrap_or_else(|| payload.claims_input.clone().unwrap_or_default());

            let abstract_text = extract_section(content, "摘要", None)
                .unwrap_or_else(|| format!("本发明公开了{}，属于{}领域。", payload.title, payload.technical_field));

            // 生成 AI prompt 用于记录
            let ai_prompt = serde_json::json!({
                "title": payload.title,
                "technical_field": payload.technical_field,
                "background_art": payload.background_art,
                "invention_description": payload.invention_description,
                "embodiments": payload.embodiments,
                "claims_input": payload.claims_input,
            });

            // 保存到数据库
            let patent_id = Uuid::new_v4();
            let now = chrono::Utc::now();

            let insert_result = sqlx::query_as::<_, PatentDocument>(
                r#"INSERT INTO patent_documents (
                    id, project_id, patent_type, title, technical_field, background_art,
                    invention_content, claims, abstract_text, embodiment, ai_prompt,
                    ai_model, version, status, created_at, updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                RETURNING
                    id, project_id, patent_type::text as patent_type, title,
                    technical_field, background_art, invention_content, claims,
                    abstract_text, drawings_description, embodiment, ai_prompt, ai_model,
                    version, status::text as status, review_comments, reviewed_by,
                    reviewed_at, created_at, updated_at"#,
            )
            .bind(patent_id)
            .bind(project_id)
            .bind(&payload.patent_type)
            .bind(&payload.title)
            .bind(&payload.technical_field)
            .bind(&payload.background_art)
            .bind(&invention_content)
            .bind(serde_json::json!({ "main": claims_text }))
            .bind(&abstract_text)
            .bind::<Option<String>>(None) // embodiment
            .bind(ai_prompt)
            .bind("claude-3-5-sonnet-20241022")
            .bind(1) // version
            .bind("draft") // status
            .bind(now)
            .bind(now)
            .fetch_one(&*state.pool)
            .await;

            match insert_result {
                Ok(patent) => {
                    // 记录 AI 使用日志
                    let _ = state
                        .ai_generator
                        .log_usage(
                            &state.pool,
                            Uuid::parse_str(&user_id).ok(),
                            None,
                            result.input_tokens,
                            result.output_tokens,
                            result.duration_ms,
                            "success",
                            None,
                        )
                        .await;

                    Json(ApiResponse::success(PatentResponse {
                        id: patent.id.to_string(),
                        project_id: patent.project_id.to_string(),
                        title: patent.title,
                        patent_type: patent.patent_type,
                        content: result.content,
                        technical_field: payload.technical_field,
                        background_art: payload.background_art,
                        invention_content,
                        claims_text,
                        abstract_text,
                        status: patent.status,
                        version: patent.version,
                        input_tokens: result.input_tokens,
                        output_tokens: result.output_tokens,
                        duration_ms: result.duration_ms,
                    }))
                }
                Err(e) => {
                    tracing::error!("Failed to save patent to database: {}", e);
                    Json(ApiResponse::error(format!("保存失败：{}", e)))
                }
            }
        }
        Err(e) => {
            let error_msg = e.to_string();

            // 记录 AI 使用日志（失败）
            let _ = state
                .ai_generator
                .log_usage(
                    &state.pool,
                    Uuid::parse_str(&user_id).ok(),
                    None,
                    0,
                    0,
                    0,
                    "error",
                    Some(&error_msg),
                )
                .await;

            Json(ApiResponse::error(format!("生成失败：{}", e)))
        }
    }
}

/// 从 AI 生成内容中提取指定章节
fn extract_section(content: &str, section_name: &str, next_section: Option<&str>) -> Option<String> {
    let section_start = content.find(&format!("{}\n", section_name))?;
    let start_idx = section_start + section_name.len() + 1;

    let remaining = &content[start_idx..];

    if let Some(next) = next_section {
        if let Some(end_idx) = remaining.find(&format!("{}\n", next)) {
            return Some(remaining[..end_idx].trim().to_string());
        }
    }

    Some(remaining.trim().to_string())
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
            updated_at"#,
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
