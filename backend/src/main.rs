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

    // 构建路由
    let app = build_app(pool, config);

    // 启动服务器
    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

fn build_app(pool: Option<Arc<sqlx::PgPool>>, config: Arc<Config>) -> Router {
    let api_routes = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/auth/login", post(api::handlers::login))
        .route("/api/auth/register", post(api::handlers::register))
        .route("/api/auth/test-account", get(api::handlers::get_test_account))
        .route("/api/projects", get(api::handlers::list_projects))
        .route("/api/patents/generate", post(api::handlers::generate_patent))
        .route("/api/copyrights/generate", post(api::handlers::generate_copyright));

    api_routes.with_state(ApiState { pool, config })
}

#[derive(Clone)]
pub struct ApiState {
    pub pool: Option<Arc<sqlx::PgPool>>,
    pub config: Arc<Config>,
}
