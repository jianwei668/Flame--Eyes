use axum::extract::State;
use axum::Json;
use crate::state::AppState;
use crate::models::dtos::GatewayStats;

pub async fn get_status(State(state): State<AppState>) -> Json<GatewayStats> {
    Json(state.stats.lock().await.clone())
}