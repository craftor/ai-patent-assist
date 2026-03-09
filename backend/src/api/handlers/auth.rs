use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::api::handlers::ApiResponse;
use crate::models::UserResponse;
use crate::auth::verify_password;
use crate::middleware::generate_token;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub user: UserResponse,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
}

// 测试账号配置
const TEST_USERNAME: &str = "admin";
const TEST_PASSWORD: &str = "admin123";

/// 登录处理函数
pub async fn login(
    State(_state): State<crate::AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    // 验证测试账号
    if payload.username == TEST_USERNAME && payload.password == TEST_PASSWORD {
        return Json(ApiResponse::success(LoginResponse {
            access_token: "test-admin-token-12345".to_string(),
            user: UserResponse {
                id: "1".to_string(),
                username: TEST_USERNAME.to_string(),
                email: "admin@patent-assist.com".to_string(),
            },
            message: "登录成功".to_string(),
        }));
    }

    // 从数据库验证用户
    match sqlx::query_as::<_, (String, String, String, String)>(
        "SELECT id::text, username, email, password_hash FROM users WHERE username = $1"
    )
    .bind(&payload.username)
    .fetch_optional(&*_state.pool)
    .await
    {
        Ok(Some((id, username, email, password_hash))) => {
            // 验证密码
            if verify_password(&payload.password, &password_hash) {
                // 生成 JWT Token
                match generate_token(&id, &username) {
                    Ok(token) => {
                        return Json(ApiResponse::success(LoginResponse {
                            access_token: token,
                            user: UserResponse {
                                id: id.clone(),
                                username,
                                email,
                            },
                            message: "登录成功".to_string(),
                        }));
                    }
                    Err(e) => {
                        tracing::error!("Failed to generate token: {}", e);
                        return Json(ApiResponse::error("Token 生成失败"));
                    }
                }
            }
        }
        Ok(None) => {
            // 用户不存在
        }
        Err(e) => {
            tracing::error!("Database error during login: {}", e);
        }
    }

    // 验证失败返回错误
    Json(ApiResponse::error("用户名或密码错误"))
}

/// 注册处理函数
pub async fn register(
    State(state): State<crate::AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Json<ApiResponse<UserResponse>> {
    use crate::auth::hash_password;
    use uuid::Uuid;

    // 生成默认密码
    let default_password = "changeit123";

    match hash_password(default_password) {
        Ok(password_hash) => {
            let user_id = Uuid::new_v4().to_string();

            // 插入新用户
            match sqlx::query_as::<_, (String, String, String)>(
                "INSERT INTO users (id, username, email, password_hash) VALUES ($1::uuid, $2, $3, $4) RETURNING id::text, username, email"
            )
            .bind(&user_id)
            .bind(&payload.username)
            .bind(&payload.email)
            .bind(&password_hash)
            .fetch_optional(&*state.pool)
            .await
            {
                Ok(Some((id, username, email))) => {
                    return Json(ApiResponse::success(UserResponse {
                        id,
                        username,
                        email,
                    }));
                }
                Ok(None) => {
                    return Json(ApiResponse::error("注册失败"));
                }
                Err(e) => {
                    tracing::error!("Database error during register: {}", e);
                    return Json(ApiResponse::error(format!("数据库错误：{}", e)));
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to hash password: {}", e);
            return Json(ApiResponse::error("密码处理失败"));
        }
    }
}

/// 获取测试账号信息
pub async fn get_test_account() -> Json<ApiResponse<TestAccountInfo>> {
    Json(ApiResponse::success(TestAccountInfo {
        username: TEST_USERNAME.to_string(),
        password: TEST_PASSWORD.to_string(),
        description: "测试管理员账号".to_string(),
    }))
}

#[derive(Debug, Serialize)]
pub struct TestAccountInfo {
    pub username: String,
    pub password: String,
    pub description: String,
}

/// 登出
pub async fn logout() -> Json<ApiResponse<()>> {
    Json(ApiResponse::success(()))
}

/// 获取当前用户 - 从 JWT Token 中获取用户信息
pub async fn get_current_user(
    State(state): State<crate::AppState>,
    ext: axum::extract::Extension<crate::middleware::Claims>,
) -> Json<ApiResponse<UserResponse>> {
    let claims = ext.0;

    // 从数据库获取用户信息
    match sqlx::query_as::<_, (String, String, String)>(
        "SELECT id::text, username, email FROM users WHERE id = $1::uuid"
    )
    .bind(&claims.sub)
    .fetch_optional(&*state.pool)
    .await
    {
        Ok(Some((id, username, email))) => {
            return Json(ApiResponse::success(UserResponse {
                id,
                username,
                email,
            }));
        }
        Ok(None) => {
            return Json(ApiResponse::error("用户不存在"));
        }
        Err(e) => {
            tracing::error!("Database error fetching current user: {}", e);
            return Json(ApiResponse::error("获取用户信息失败"));
        }
    }
}
