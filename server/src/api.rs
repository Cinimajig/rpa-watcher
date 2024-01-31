use crate::state::AppState;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use rpa::RpaData;
use tokio::{sync::RwLock, time::Instant};

type ApiState = AppState<Vec<RpaData>>;
type FailedState = Vec<(Instant, RpaData)>;

/// List of failed RPA-tasks. These will live for a day and then be cleared.
/// Right now it's unsued.
pub static mut FAILED_RPADATA: RwLock<FailedState> = RwLock::const_new(FailedState::new());

pub fn router() -> Router {
    let buffer_data: ApiState = ApiState::new(Vec::with_capacity(10));

    Router::new()
        .route("/getrpa", get(get_rpadata))
        .route("/checkin", post(post_checkin))
        .with_state(buffer_data)
        .fallback(crate::fallback)
}

async fn get_rpadata(
    // headers: HeaderMap, 
    State(state): State<ApiState>
) -> Json<Vec<RpaData>> {
    let data = state.data.read().await;
    Json(data.clone())
}

async fn post_checkin(
    // headers: HeaderMap,
    State(state): State<ApiState>,
    Json(payload): Json<Vec<RpaData>>,
) -> StatusCode {
    if payload.is_empty() {
        return StatusCode::NO_CONTENT
    }

    let mut data = state.data.write().await;
    payload.into_iter().for_each(|item| data.push(item));
    
    StatusCode::OK
}
