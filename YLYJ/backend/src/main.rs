use axum::{routing::{get}, Router};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;
use dotenvy::dotenv;

mod config;
mod handlers;
mod models;
mod services;
mod state;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. 初始化环境
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    dotenv().ok();

    let config = config::AppConfig::from_env();

    // 2. 构建全局状态
    let app_state = state::AppState {
        device_name: config.device_name.clone(),
        stats: Arc::new(Mutex::new(models::dtos::GatewayStats {
            captured_count: 0,
            anonymized_count: 0,
            sm2_signed_count: 0,
            pushed_to_cloud: 0,
            eth0_status: "Monitoring".into(),
            eth1_status: "TLS Outbound".into(),
            last_processed_at: None,
        })),
    };

    // 3. 启动后台流水线
    tokio::spawn(services::pipeline::run_pipeline(app_state.clone()));

    // 4. 路由配置
    let app = Router::new()
        .route("/api/health", get(handlers::health::health))
        .route("/api/device/status", get(handlers::status::get_status))
        .with_state(app_state)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    // 5. 启动
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("🚀 Gateway running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}