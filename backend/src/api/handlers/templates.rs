use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::handlers::ApiResponse;
use crate::AppState;

/// 模板数据结构
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DocumentTemplate {
    pub id: Uuid,
    pub name: String,
    pub template_type: String,
    pub content_template: String,
    pub variables: Option<serde_json::Value>,
    pub description: Option<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub template_type: String,
    pub content_template: String,
    pub variables: Option<serde_json::Value>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub template_type: Option<String>,
    pub content_template: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

/// 获取单个模板详情
pub async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DocumentTemplate>>, StatusCode> {
    let template = sqlx::query_as::<_, DocumentTemplate>(
        r#"SELECT
            id,
            name,
            type as template_type,
            content_template,
            variables,
            description,
            is_system,
            is_active,
            created_by,
            created_at,
            updated_at
        FROM document_templates
        WHERE id = $1"#
    )
    .bind(id)
    .fetch_optional(&*state.pool)
    .await;

    match template {
        Ok(Some(template)) => Ok(Json(ApiResponse::success(template))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get template: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 列出模板
pub async fn list_templates(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<DocumentTemplate>>> {
    let templates = sqlx::query_as::<_, DocumentTemplate>(
        r#"SELECT
            id,
            name,
            type as template_type,
            content_template,
            variables,
            description,
            is_system,
            is_active,
            created_by,
            created_at,
            updated_at
        FROM document_templates
        WHERE is_active = true
        ORDER BY is_system DESC, created_at DESC"#
    )
    .fetch_all(&*state.pool)
    .await;

    match templates {
        Ok(templates) => Json(ApiResponse::success(templates)),
        Err(e) => {
            tracing::error!("Failed to list templates: {}", e);
            Json(ApiResponse::error("Failed to list templates"))
        }
    }
}

/// 创建模板
pub async fn create_template(
    State(state): State<AppState>,
    Json(payload): Json<CreateTemplateRequest>,
) -> Json<ApiResponse<DocumentTemplate>> {
    let template = sqlx::query_as::<_, DocumentTemplate>(
        r#"INSERT INTO document_templates
            (name, type, content_template, variables, description, is_system, is_active)
        VALUES ($1, $2, $3, $4, $5, false, COALESCE($6, true))
        RETURNING
            id,
            name,
            type as template_type,
            content_template,
            variables,
            description,
            is_system,
            is_active,
            created_by,
            created_at,
            updated_at"#
    )
    .bind(&payload.name)
    .bind(&payload.template_type)
    .bind(&payload.content_template)
    .bind(&payload.variables)
    .bind(&payload.description)
    .bind(&payload.is_active)
    .fetch_optional(&*state.pool)
    .await;

    match template {
        Ok(Some(template)) => Json(ApiResponse::success(template)),
        Ok(None) => Json(ApiResponse::error("创建失败")),
        Err(e) => {
            tracing::error!("Failed to create template: {}", e);
            Json(ApiResponse::error(format!("创建失败：{}", e)))
        }
    }
}

/// 更新模板
pub async fn update_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTemplateRequest>,
) -> Result<Json<ApiResponse<DocumentTemplate>>, StatusCode> {
    let updated = sqlx::query_as::<_, DocumentTemplate>(
        r#"UPDATE document_templates SET
            name = COALESCE($2, name),
            type = COALESCE($3, type),
            content_template = COALESCE($4, content_template),
            variables = COALESCE($5, variables),
            description = COALESCE($6, description),
            is_active = COALESCE($7, is_active),
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            name,
            type as template_type,
            content_template,
            variables,
            description,
            is_system,
            is_active,
            created_by,
            created_at,
            updated_at"#
    )
    .bind(id)
    .bind(payload.name)
    .bind(payload.template_type)
    .bind(payload.content_template)
    .bind(payload.variables)
    .bind(payload.description)
    .bind(payload.is_active)
    .fetch_optional(&*state.pool)
    .await;

    match updated {
        Ok(Some(template)) => Ok(Json(ApiResponse::success(template))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update template: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 删除模板
pub async fn delete_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM document_templates WHERE id = $1"
    )
    .bind(id)
    .execute(&*state.pool)
    .await;

    match result {
        Ok(rows) => {
            if rows.rows_affected() > 0 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete template: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
