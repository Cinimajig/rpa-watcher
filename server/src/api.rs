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

async fn get_rpadata(// headers: HeaderMap,
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
        let mut value = RpaValue::new(now, item);

        // Search the PR database for a name.
        match crate::db::ProcessRobotJob::query_instance(&value.data.instance).await {
            Ok(pr) => {
                value.data.flow_id = Some(pr.job_name);
                value.data.trigger = Some(rpa::RpaTrigger::Custom(pr.cause_text));
            },
            Err(err) => {
                if cfg!(debug_assertions) {
                    eprintln!("Failed to find ProcessRobot job. {err}");
                }
            },
        }

        data.insert(value.data.instance.clone(), value);
    }

    StatusCode::OK
}

pub async fn cleanup_timer() {
    use tokio::time::*;

    const TIMEOUT: u64 = 32;

    loop {
        sleep(Duration::from_secs(CLEANUP_TIMER_INTERVAL)).await;


        let mut data = success_rpa_mut().await;
        data.retain(|k, v| {
            let secs = v.timestamp.elapsed().as_secs();

            #[cfg(debug_assertions)]
            if secs >= TIMEOUT {
                println!("Cleaning up {k}");
            }

            secs < TIMEOUT
        });
    }
}
