use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::api::handlers::ApiResponse;
use crate::AppState;

/// 列出 AI 模型
pub async fn list_models(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ModelResponse>>>, StatusCode> {
    let models: Vec<ModelResponse> = sqlx::query_as(
        r#"SELECT id, provider, model_name, COALESCE(is_default, false) as is_default
           FROM ai_model_configs
           ORDER BY is_default DESC, priority ASC, created_at DESC"#,
    )
    .fetch_all(&*state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to list models: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ApiResponse::success(models)))
}

/// 添加模型
pub async fn add_model(
    State(state): State<AppState>,
    Json(payload): Json<AddModelRequest>,
) -> Result<Json<ApiResponse<ModelResponse>>, StatusCode> {
    // 检查是否是第一个模型，如果是则设为默认
    let is_first_model =
        sqlx::query_scalar::<_, bool>("SELECT NOT EXISTS(SELECT 1 FROM ai_model_configs)")
            .fetch_one(&*state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
            .unwrap_or(true);

    let model: ModelResponse = sqlx::query_as(
        r#"INSERT INTO ai_model_configs (provider, model_name, api_key_encrypted, is_default)
           VALUES ($1, $2, $3, $4)
           RETURNING id, provider, model_name, is_default"#,
    )
    .bind(&payload.provider)
    .bind(&payload.model_name)
    .bind(&payload.api_key)
    .bind(is_first_model)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to add model: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ApiResponse::success(model)))
}

/// 更新模型
pub async fn update_model(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateModelRequest>,
) -> Result<Json<ApiResponse<ModelResponse>>, StatusCode> {
    let model: ModelResponse = sqlx::query_as(
        r#"UPDATE ai_model_configs
           SET provider = COALESCE($1, provider),
               model_name = COALESCE($2, model_name),
               api_key_encrypted = COALESCE($3, api_key_encrypted)
           WHERE id = $4
           RETURNING id, provider, model_name, is_default"#,
    )
    .bind(&payload.provider)
    .bind(&payload.model_name)
    .bind(&payload.api_key)
    .bind(&id)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update model: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ApiResponse::success(model)))
}

/// 删除模型
pub async fn delete_model(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // 检查是否是默认模型，如果是则不允许删除
    let is_default =
        sqlx::query_scalar::<_, bool>("SELECT is_default FROM ai_model_configs WHERE id = $1")
            .bind(&id)
            .fetch_optional(&*state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .unwrap_or(false);

    if is_default {
        return Err(StatusCode::FORBIDDEN);
    }

    sqlx::query("DELETE FROM ai_model_configs WHERE id = $1")
        .bind(&id)
        .execute(&*state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete model: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}

/// 获取默认模型
pub async fn get_default_model(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ModelResponse>>, StatusCode> {
    let model: ModelResponse = sqlx::query_as(
        r#"SELECT id, provider, model_name, is_default
           FROM ai_model_configs
           WHERE is_default = true
           LIMIT 1"#,
    )
    .fetch_one(&*state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse::success(model)))
}

/// 设置默认模型
pub async fn set_default_model(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ModelResponse>>, StatusCode> {
    // 检查模型是否存在
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM ai_model_configs WHERE id = $1)",
    )
    .bind(&id)
    .fetch_one(&*state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !exists {
        return Err(StatusCode::NOT_FOUND);
    }

    // 清除所有默认标记
    sqlx::query("UPDATE ai_model_configs SET is_default = false")
        .execute(&*state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 设置新的默认模型
    let model: ModelResponse = sqlx::query_as(
        r#"UPDATE ai_model_configs SET is_default = true
           WHERE id = $1
           RETURNING id, provider, model_name, is_default"#,
    )
    .bind(&id)
    .fetch_one(&*state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse::success(model)))
}

#[derive(Debug, serde::Deserialize)]
pub struct AddModelRequest {
    pub provider: String,
    pub model_name: String,
    pub api_key: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateModelRequest {
    pub provider: Option<String>,
    pub model_name: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct ModelResponse {
    pub id: Uuid,
    pub provider: String,
    pub model_name: String,
    pub is_default: bool,
}
