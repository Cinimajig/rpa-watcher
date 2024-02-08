use crate::state::AppState;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use rpa::RpaData;
use tokio::{sync::{RwLock, RwLockReadGuard}, time::Instant};

type ApiState = AppState<Vec<RpaData>>;
type FailedState = Vec<(Instant, RpaData)>;

/// List of failed RPA-tasks. These will live for a day and then be cleared.
/// Right now it's unsued.
pub static mut FAILED_RPADATA: RwLock<FailedState> = RwLock::const_new(FailedState::new());

#[inline]
pub async fn read_failed_rpadata() -> RwLockReadGuard<'static, FailedState> {
    unsafe {
        FAILED_RPADATA.read().await
    }
}

pub fn router() -> Router {
    let buffer_data: ApiState = ApiState::new(Vec::with_capacity(10));

    Router::new()
        .route("/getrpa", get(get_rpadata))
        .route("/getfailed", get(get_failed_rpadata))
        .route("/checkin", post(post_checkin))
        .with_state(buffer_data)
        .fallback(crate::fallback)
}

async fn get_failed_rpadata(
    // headers: HeaderMap,
) -> Json<Vec<RpaData>> {
    let data = read_failed_rpadata().await;
    Json(data.iter().map(|r| r.1.clone()).collect())
}

async fn get_rpadata(
    // headers: HeaderMap, 
    State(state): State<ApiState>
) -> Json<Vec<RpaData>> {

    let data = state.data.read().await;

    #[cfg(debug_assertions)]
    println!("Sending {} items", data.len());

    Json(data.clone())
}

async fn post_checkin(
    // headers: HeaderMap,
    State(state): State<ApiState>,
    Json(payload): Json<Vec<RpaData>>,
) -> StatusCode {
    #[cfg(debug_assertions)]
    println!("Recieved packet: {:?}", payload);
    
    if payload.is_empty() {
        return StatusCode::NO_CONTENT
    }

    #[cfg(debug_assertions)]
    println!("\tAdded to state");

    let mut data = state.data.write().await;
    payload.into_iter().for_each(|item| data.push(item));
    
    StatusCode::OK
}
