use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GatewayStats {
    pub captured_count: u64,
    pub anonymized_count: u64,
    pub sm2_signed_count: u64,
    pub pushed_to_cloud: u64,
    pub eth0_status: String,
    pub eth1_status: String,
    pub last_processed_at: Option<String>,
}