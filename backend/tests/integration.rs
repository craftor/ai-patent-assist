// 后端集成测试
// 运行测试：cargo test --test integration

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;

// 导入被测试的模块
use ai_patent_assist_backend::{build_app, config::Config, services::AiGenerator, AppState};
use sqlx::PgPool;
use std::sync::Arc;

/// 健康检查测试
#[tokio::test]
async fn test_health_check() {
    // 创建测试应用（不需要真实的数据库连接）
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

/// 登录接口测试 - 使用测试账号
#[tokio::test]
async fn test_login_with_test_account() {
    let app = create_test_app().await;

    let login_payload = r#"{"username":"admin","password":"admin123"}"#;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(login_payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&body).unwrap();

    // 验证响应结构
    assert_eq!(json["success"], true);
    assert!(json["data"]["access_token"].is_string());
    assert_eq!(json["data"]["user"]["username"], "admin");
}

/// 未授权访问测试
#[tokio::test]
async fn test_unauthorized_access() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // 没有 Token 应该返回 401
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

/// 使用测试 Token 访问受保护端点
#[tokio::test]
async fn test_protected_endpoint_with_test_token() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .header("Authorization", "Bearer test-admin-token-12345")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["username"], "admin");
}

/// 获取专利列表测试
#[tokio::test]
async fn test_list_patents() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/patents")
                .header("Authorization", "Bearer test-admin-token-12345")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    assert!(json["data"].is_array());
}

/// 获取项目列表测试
#[tokio::test]
async fn test_list_projects() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/projects")
                .header("Authorization", "Bearer test-admin-token-12345")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    assert!(json["data"].is_array());
}

/// 获取 AI 模型列表测试
#[tokio::test]
async fn test_list_ai_models() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ai/models")
                .header("Authorization", "Bearer test-admin-token-12345")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["success"], true);
    assert!(json["data"].is_array());
}

/// 创建测试应用
async fn create_test_app() -> axum::Router {
    // 使用测试数据库 URL
    let database_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        "postgres://patent_user:patent_password@localhost:5432/patent_db".to_string()
    });

    // 尝试创建数据库连接池，如果失败则使用 mock
    let pool = match PgPool::connect(&database_url).await {
        Ok(p) => Arc::new(p),
        Err(_) => {
            // 如果数据库不可用，跳过需要数据库的测试
            eprintln!("Database not available, some tests may be skipped");
            Arc::new(PgPool::connect_lazy("postgres://localhost/test").unwrap())
        }
    };

    // 创建配置
    let config = Arc::new(create_test_config(&database_url));

    // 创建 AI 生成器
    let ai_generator = Arc::new(AiGenerator::new(config.as_ref().clone()));

    // 构建应用
    build_app(AppState {
        pool,
        config,
        ai_generator,
    })
}

/// 创建测试配置
fn create_test_config(database_url: &str) -> Config {
    Config {
        database_url: database_url.to_string(),
        jwt_secret: "test-jwt-secret".to_string(),
        jwt_expiry_hours: 24,
        server_port: 3000,
        anthropic_api_key: "test-key".to_string(),
    }
}
