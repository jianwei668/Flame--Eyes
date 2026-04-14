use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::dtos::GatewayStats;

#[derive(Clone)]
pub struct AppState {
    // 设备统计信息（内存级，高频读写）
    pub stats: Arc<Mutex<GatewayStats>>,
    
    pub device_name: String,
}