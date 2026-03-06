mod api;
mod auth;
mod config;
mod db;
mod models;
mod services;

use axum::{routing::{get, post, put, delete}, Router};
use tokio::net::TcpListener;
use std::sync::Arc;

use crate::config::Config;
use crate::db::create_pool;
use crate::services::AiGenerator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let config = Config::load()?;
    tracing::info!("Loaded config");
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

fn build_app(state: AppState) -> Router {
    let api_routes = Router::new()
        // 健康检查
        .route("/health", get(|| async { "OK" }))
        // 认证相关
        .route("/api/auth/login", post(api::handlers::login))
        .route("/api/auth/register", post(api::handlers::register))
        .route("/api/auth/test-account", get(api::handlers::get_test_account))
        .route("/api/auth/me", get(api::handlers::get_current_user))
        .route("/api/auth/logout", post(api::handlers::logout))
        // 用户管理
        .route("/api/users", get(api::handlers::list_users))
        .route("/api/users/:id", get(api::handlers::get_user))
        .route("/api/users/:id", put(api::handlers::update_user))
        .route("/api/users/:id", delete(api::handlers::delete_user))
        // 项目管理
        .route("/api/projects", get(api::handlers::list_projects))
        .route("/api/projects", post(api::handlers::create_project))
        .route("/api/projects/:id", get(api::handlers::get_project))
        .route("/api/projects/:id", put(api::handlers::update_project))
        .route("/api/projects/:id", delete(api::handlers::delete_project))
        .route("/api/projects/:id/attachments", post(api::handlers::upload_attachment))
        .route("/api/projects/:id/attachments/:file_id", delete(api::handlers::delete_attachment))
        // 专利管理
        .route("/api/patents", get(api::handlers::list_patents))
        .route("/api/patents/:id", get(api::handlers::get_patent))
        .route("/api/patents/:id", put(api::handlers::update_patent))
        .route("/api/patents/generate", post(api::handlers::generate_patent))
        // 软著管理
        .route("/api/copyrights", get(api::handlers::list_copyrights))
        .route("/api/copyrights/:id", get(api::handlers::get_copyright))
        .route("/api/copyrights/:id", put(api::handlers::update_copyright))
        .route("/api/copyrights/generate", post(api::handlers::generate_copyright));

    api_routes.with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<sqlx::PgPool>,
    pub config: Arc<Config>,
    pub ai_generator: Arc<AiGenerator>,
}

