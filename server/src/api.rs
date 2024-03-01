use std::sync::Arc;

use crate::rpa_state::*;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use rpa::RpaData;
use tokio::{sync::RwLock, time::Instant};

const CLEANUP_TIMER_INTERVAL: u64 = 5;

#[derive(Clone)]
pub struct RpaState {
    pub success: Arc<RwLock<RpaItems>>,
    pub failed: Arc<RwLock<RpaItems>>,
}

impl RpaState {
    pub fn new(sucess: RpaItems, failed: RpaItems) -> Self {
        Self {
            success: Arc::new(RwLock::new(sucess)),
            failed: Arc::new(RwLock::new(failed)),
        }
    }
}

pub fn router() -> Router {
    // let buffer_data: RpaState = RpaState::new(
    //     HashMap::with_capacity(10), HashMap::with_capacity(20)
    // );

    Router::new()
        .route("/getrpa", get(get_rpadata))
        .route("/getfailed", get(get_failed_rpadata))
        .route("/checkin", post(post_checkin))
        // .with_state(buffer_data)
        .fallback(crate::fallback)
}

async fn get_failed_rpadata(// headers: HeaderMap,
    // State(state): State<RpaState>,
) -> Json<Vec<RpaData>> {
    let data = rpa_f().await;
    Json(data.iter().map(|(_k, v)| v.1.clone()).collect())
}

async fn get_rpadata(// headers: HeaderMap,
    // State(state): State<RpaState>,
) -> Json<Vec<RpaData>> {
    let data = rpa_s().await;

    #[cfg(debug_assertions)]
    println!("Sending {} items", data.len());

    Json(data.iter().map(|(_k, v)| v.1.clone()).collect())
}

async fn post_checkin(
    // headers: HeaderMap,
    // State(state): State<RpaState>,
    Json(payload): Json<Vec<RpaData>>,
) -> StatusCode {
    #[cfg(debug_assertions)]
    println!("Recieved payload: {:?}", payload);

    if payload.is_empty() {
        return StatusCode::BAD_REQUEST;
    }

    #[cfg(debug_assertions)]
    println!("\tAdded to state");

    let now = Instant::now();
    let mut data = rpa_s_mut().await;
    for item in payload.into_iter() {
        data.insert(item.instance.clone(), (now, item));
    }

    StatusCode::OK
}

pub async fn cleanup_timer() {
    use tokio::time::*;

    loop {
        sleep(Duration::from_secs(CLEANUP_TIMER_INTERVAL)).await;
        
        #[cfg(debug_assertions)]
        println!("Cleaning up...");

        let mut data = rpa_s_mut().await;
        data.retain(|_k, v| v.0.elapsed().as_secs() < 60);        
    }
}
