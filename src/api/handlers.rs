use axum::{extract::State, response::Json, http::StatusCode};
use serde_json::{json, Value};
type ApiState = (String, crate::config::UnifiedConfig);
pub async fn status(State((_api_key, config)): State<ApiState>) -> Json<Value> {
    Json(json!({ "mode": config.mode, "control_api": "running" }))
}
pub async fn config(State((_api_key, config)): State<ApiState>) -> Json<Value> {
    Json(serde_json::to_value(&config).unwrap())
}
pub async fn metrics() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "metrics": "stub" })))
}
