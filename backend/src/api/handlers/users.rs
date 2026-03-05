use axum::{extract::{State, Path}, http::StatusCode, Json};
use uuid::Uuid;

use crate::api::handlers::ApiResponse;
use crate::models::User;

/// 获取用户列表
pub async fn list_users(
    State(state): State<crate::AppState>,
) -> Result<Json<ApiResponse<Vec<User>>>, StatusCode> {
    let pool = &*state.pool;

    let users: Vec<User> = sqlx::query_as(
        r#"SELECT * FROM users ORDER BY created_at DESC"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse::success(users)))
}

/// 获取用户详情
pub async fn get_user(
    State(state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let pool = &*state.pool;

    let user: User = sqlx::query_as(
        r#"SELECT * FROM users WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse::success(user)))
}

/// 更新用户
pub async fn update_user(
    State(state): State<crate::AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let pool = &*state.pool;

    let user: User = sqlx::query_as(
        r#"UPDATE users SET
            full_name = COALESCE($1, full_name),
            avatar_url = COALESCE($2, avatar_url),
            email = COALESCE($3, email)
        WHERE id = $4 RETURNING *"#,
    )
    .bind(payload.full_name)
    .bind(payload.avatar_url)
    .bind(payload.email)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse::success(user)))
}

/// 删除用户
pub async fn delete_user(
    State(state): State<crate::AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let pool = &*state.pool;

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
}
