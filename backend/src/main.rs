mod api;
mod auth;
mod config;
mod db;
mod models;
mod services;

use axum::{routing::{get, post}, Router};
use tokio::net::TcpListener;
use tracing_subscriber::fmt;
use std::sync::Arc;

use crate::config::Config;
use crate::db::create_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    fmt::init();

    // 加载配置
    let config = Config::load()?;
    let config = Arc::new(config);

    // 创建数据库连接池（如果连接失败则继续运行）
    let pool = create_pool(&config.database_url).await.ok();
    let pool = pool.map(Arc::new);

    // 构建服务器状态
    let state = AppState {
        pool: pool.clone(),
        config: config.clone(),
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
        .route("/api/users/:id", axum::routing::put(api::handlers::update_user))
        .route("/api/users/:id", axum::routing::delete(api::handlers::delete_user))
        // 项目管理
        .route("/api/projects", get(api::handlers::list_projects))
        .route("/api/projects", post(api::handlers::create_project))
        .route("/api/projects/:id", get(api::handlers::get_project))
        .route("/api/projects/:id", axum::routing::put(api::handlers::update_project))
        .route("/api/projects/:id", axum::routing::delete(api::handlers::delete_project))
        .route("/api/projects/:id/attachments", post(api::handlers::upload_attachment))
        .route("/api/projects/:id/attachments/:file_id", axum::routing::delete(api::handlers::delete_attachment))
        // 专利生成
        .route("/api/patents/generate", post(api::handlers::generate_patent))
        // 软著生成
        .route("/api/copyrights/generate", post(api::handlers::generate_copyright));

    api_routes.with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Option<Arc<sqlx::PgPool>>,
    pub config: Arc<Config>,
}
