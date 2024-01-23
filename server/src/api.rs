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

    Router::new()
        .route("/getrpa", get(get_rpadata))
        .route("/checkin", post(post_checkin))
        .with_state(buffer_data)
        .fallback(crate::fallback)
}

async fn get_rpadata(headers: HeaderMap, State(state): State<ApiState>) -> Json<Vec<RpaData>> {
    println!("headers: {headers:?}");

    let data = state.data.lock().await;
    Json(data.clone())
}

async fn post_checkin(
    headers: HeaderMap,
    State(state): State<ApiState>,
    Json(payload): Json<Vec<RpaData>>,
) -> StatusCode {
    let mut data = state.data.lock().await;
    payload.into_iter().for_each(|item| data.push(item));
    
    StatusCode::OK
}
