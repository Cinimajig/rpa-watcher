use crate::rpa_state::*;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use rpa::RpaData;
use tokio::time::Instant;

const CLEANUP_TIMER_INTERVAL: u64 = 5;

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
    let data = rpa_failed().await;
    Json(data.iter().map(|(_k, v)| v.data.clone()).collect())
}

async fn get_rpadata(
    // headers: HeaderMap,
    // State(state): State<RpaState>,
) -> Json<Vec<RpaData>> {
    let data = success_rpa().await;

    #[cfg(debug_assertions)]
    println!("Sending {} items", data.len());

    Json(data.iter().map(|(_k, v)| v.data.clone()).collect())
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
    let mut data = success_rpa_mut().await;
    for item in payload.into_iter() {
        data.insert(item.instance.clone(), RpaValue::new(now, item));
    }

    StatusCode::OK
}

pub async fn cleanup_timer() {
    use tokio::time::*;

    loop {
        sleep(Duration::from_secs(CLEANUP_TIMER_INTERVAL)).await;
        
        #[cfg(debug_assertions)]
        println!("Cleaning up...");

        let mut data = success_rpa_mut().await;
        data.retain(|_k, v| v.timestamp.elapsed().as_secs() < 60);        
    }
}
