use axum::Json;
use serde::{Deserialize, Serialize};

use crate::api::handlers::ApiResponse;
use crate::models::UserResponse;

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
    pub password: String,
}

// 测试账号配置
const TEST_USERNAME: &str = "admin";
const TEST_PASSWORD: &str = "admin123";

pub async fn login(Json(payload): Json<LoginRequest>) -> Json<ApiResponse<LoginResponse>> {
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

    // 其他账号返回错误
    Json(ApiResponse::error("用户名或密码错误"))
}

pub async fn register(Json(payload): Json<RegisterRequest>) -> Json<ApiResponse<UserResponse>> {
    // 简化版本
    Json(ApiResponse::success(UserResponse {
        id: "1".to_string(),
        username: payload.username,
        email: payload.email,
    }))
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
