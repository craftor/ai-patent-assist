mod api;
mod auth;
pub mod config;
mod db;
mod middleware;
mod models;
pub mod services;

use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::config::Config;
use crate::db::create_pool;
use crate::services::AiGenerator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let config = Config::load()?;
    let is_dev = config.is_dev_environment();
    tracing::info!("配置加载成功 - 环境：{}", if is_dev { "开发" } else { "生产" });

    // 显示配置信息（敏感信息脱敏）
    tracing::info!("  - 数据库：{}", mask_database_url(&config.database_url));
    tracing::info!("  - JWT 密钥：{} 字符", config.jwt_secret.len());
    tracing::info!("  - JWT 过期：{} 小时", config.jwt_expiry_hours);
    tracing::info!("  - 服务端口：{}", config.server_port);
    tracing::info!("  - API Key: {}***", &config.anthropic_api_key.chars().take(5).collect::<String>());

    let config = Arc::new(config);

    // 创建数据库连接池
    let pool = create_pool(&config.database_url).await?;
    tracing::info!("Database pool created");
    let pool = Arc::new(pool);

    // 初始化 AI 生成器
    let ai_generator = AiGenerator::new(config.as_ref().clone());
    let ai_generator = Arc::new(ai_generator);

    // 构建服务器状态
    let state = AppState {
        pool: pool.clone(),
        config: config.clone(),
        ai_generator: ai_generator.clone(),
    };

    // 构建路由
    let app = build_app(state);

    // 启动服务器
    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

pub fn build_app(state: AppState) -> Router {
    // 公开路由（不需要认证）
    let public_routes = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/health", get(|| async { "OK" }))
        .route("/api/auth/login", post(api::handlers::login))
        .route("/api/auth/register", post(api::handlers::register))
        .route(
            "/api/auth/test-account",
            get(api::handlers::get_test_account),
        );

    // 受保护的路由（需要认证）
    let protected_routes = Router::new()
        // 用户管理
        .route("/api/auth/me", get(api::handlers::get_current_user))
        .route("/api/auth/logout", post(api::handlers::logout))
        .route("/api/users", get(api::handlers::list_users))
        .route("/api/users/{id}", get(api::handlers::get_user))
        .route("/api/users/{id}", put(api::handlers::update_user))
        .route("/api/users/{id}", delete(api::handlers::delete_user))
        .route(
            "/api/users/{id}/password",
            put(api::handlers::change_password),
        )
        // 项目管理
        .route("/api/projects", get(api::handlers::list_projects))
        .route("/api/projects", post(api::handlers::create_project))
        .route("/api/projects/{id}", get(api::handlers::get_project))
        .route("/api/projects/{id}", put(api::handlers::update_project))
        .route("/api/projects/{id}", delete(api::handlers::delete_project))
        .route(
            "/api/projects/{id}/attachments",
            post(api::handlers::upload_attachment),
        )
        .route(
            "/api/projects/{id}/attachments/{file_id}",
            delete(api::handlers::delete_attachment),
        )
        // 专利管理
        .route("/api/patents", get(api::handlers::list_patents))
        .route("/api/patents/{id}", get(api::handlers::get_patent))
        .route("/api/patents/{id}", put(api::handlers::update_patent))
        .route(
            "/api/patents/generate",
            post(api::handlers::generate_patent),
        )
        // 软著管理
        .route("/api/copyrights", get(api::handlers::list_copyrights))
        .route("/api/copyrights/{id}", get(api::handlers::get_copyright))
        .route("/api/copyrights/{id}", put(api::handlers::update_copyright))
        .route(
            "/api/copyrights/generate",
            post(api::handlers::generate_copyright),
        )
        // 模板管理
        .route("/api/templates", get(api::handlers::list_templates))
        .route("/api/templates", post(api::handlers::create_template))
        .route("/api/templates/{id}", get(api::handlers::get_template))
        .route("/api/templates/{id}", put(api::handlers::update_template))
        .route(
            "/api/templates/{id}",
            delete(api::handlers::delete_template),
        )
        // AI 模型管理
        .route("/api/ai/models", get(api::handlers::list_models))
        .route("/api/ai/models", post(api::handlers::add_model))
        .route("/api/ai/models/{id}", put(api::handlers::update_model))
        .route("/api/ai/models/{id}", delete(api::handlers::delete_model))
        .route(
            "/api/ai/models/{id}/set-default",
            post(api::handlers::set_default_model),
        )
        .route(
            "/api/ai/models/default",
            get(api::handlers::get_default_model),
        );

    // 将公开路由和受保护路由合并，并添加认证中间件到受保护路由
    public_routes
        .merge(protected_routes.layer(from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        )))
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<sqlx::PgPool>,
    pub config: Arc<Config>,
    pub ai_generator: Arc<AiGenerator>,
}

/// 脱敏数据库 URL（用于日志）
fn mask_database_url(url: &str) -> String {
    // 简单实现：只显示协议和主机
    if let Some(start) = url.find("://") {
        let protocol = &url[..start + 3];
        let rest = &url[start + 3..];
        if let Some(at) = rest.find('@') {
            // 有认证信息
            if let Some(slash) = rest[at..].find('/') {
                format!("{}***@{}***", protocol, &rest[at + 1..at + slash])
            } else {
                format!("{}***@{}", protocol, &rest[at + 1..])
            }
        } else {
            format!("{}{}", protocol, rest)
        }
    } else {
        "***".to_string()
    }
}
