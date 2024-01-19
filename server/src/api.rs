use crate::state::AppState;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use rpa::RpaData;

type ApiState = AppState<Vec<RpaData>>;

pub fn router() -> Router {
    let buffer_data: ApiState = AppState::new(Vec::with_capacity(10));

    println!(
        "{}",
        serde_json::to_string_pretty(&RpaData {
            pid: 1234,
            engine: rpa::RpaEngine::PowerAutomate,
            computer: "Desktop".to_string(),
            env: Some("12312313".to_string()),
            instance: "sadsadasdasd".to_string(),
            azure_data: None
        }).unwrap()
    );

    Router::new()
        .route("/getrpa", get(get_rpadata))
        .route("/checkin", post(post_checkin))
        .with_state(buffer_data)
}

pub async fn get_rpadata(headers: HeaderMap, State(state): State<ApiState>) -> Json<Vec<RpaData>> {
    println!("headers: {headers:?}");

    let data = state.data.lock().await;
    Json(data.clone())
}

pub async fn post_checkin(
    headers: HeaderMap,
    State(state): State<ApiState>,
    Json(payload): Json<Vec<RpaData>>,
) -> StatusCode {
    let mut data = state.data.lock().await;
    for item in payload {
        data.push(item);
    }

    StatusCode::OK
}
