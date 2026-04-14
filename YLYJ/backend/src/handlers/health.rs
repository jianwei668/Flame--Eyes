use axum::extract::State;
use axum::Json;
use crate::state::AppState;

pub async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "running",
        "device": state.device_name,
        "topology": "LIS/HIS -> [ETH0:SPAN] -> Gateway -> [ETH1:TLS] -> Cloud"
    }))
}