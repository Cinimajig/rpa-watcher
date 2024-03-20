use crate::rpa_state::*;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use rpa::RpaData;
use std::sync::Arc;
use tokio::{sync::RwLock, time::Instant};

// type MaybeDatabase = Option<Arc<RwLock<db::Database>>>;

const CLEANUP_TIMER_INTERVAL: u64 = 5;
const CLEANUP_TIMEOUT: u64 = 32;

#[derive(Clone)]
pub struct GLobalState {
    pub prdb: Option<Arc<RwLock<crate::db::Database>>>,
    pub paapi: Option<Arc<RwLock<crate::pa_api::PowerAutomateAPI>>>,
}

pub fn router(database: GLobalState) -> Router {
    // let buffer_data: RpaState = RpaState::new(
    //     HashMap::with_capacity(10), HashMap::with_capacity(20)
    // );

    Router::new()
        .route("/getrpa", get(get_rpadata))
        .route("/getfailed", get(get_failed_rpadata))
        .route("/checkin", post(post_checkin))
        .with_state(database)
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
    State(state): State<GLobalState>,
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
        let instance = item.instance.clone();

        // If the item already exist, then update the timestamp.
        if let Some(rpa_data) = data.get_mut(&instance) {
            rpa_data.timestamp = now;
        } else {
            // Otherwise create a new value and try to lookup it's name.
            let mut value = RpaValue::new(now, item);

            match value.data.engine {
                rpa::RpaEngine::PowerAutomate => {
                    if let Some(paapi) = state.clone().paapi {
                        let mut client = paapi.write().await;
                        let Some(id) = value.data.flow_id.as_ref() else {
                            if cfg!(debug_assertions) {
                                eprintln!(
                                    "didn't have flow id for instance: {}",
                                    &value.data.instance
                                );
                            }
                            break;
                        };
                        match crate::pa_api::lookup_uiflow(&mut client, id).await {
                            Ok(flow_name) => value.data.flow_id = Some(flow_name),
                            Err(err) => {
                                if cfg!(debug_assertions) {
                                    eprintln!("Failed to find ProcessRobot job. {err}");
                                }
                            }
                        }
                    }
                }
                rpa::RpaEngine::ProcessRobot => {
                    // Search the PR database for a name.
                    if let Some(db_client) = state.clone().prdb {
                        let mut client = db_client.write().await;
                        match crate::db::ProcessRobotJob::query_instance(
                            &mut client,
                            &value.data.instance,
                        )
                        .await
                        {
                            Ok(pr) => {
                                value.data.flow_id = Some(pr.job_name);
                                value.data.trigger = Some(rpa::RpaTrigger::Custom(pr.cause_text));
                            }
                            Err(err) => {
                                if cfg!(debug_assertions) {
                                    eprintln!("Failed to find ProcessRobot job. {err}");
                                }
                            }
                        }
                    }
                }
            }
            data.insert(instance, value);
        }
    }

    StatusCode::OK
}

pub async fn cleanup_timer() {
    use tokio::time::*;

    loop {
        sleep(Duration::from_secs(CLEANUP_TIMER_INTERVAL)).await;

        let mut data = success_rpa_mut().await;
        data.retain(|k, v| {
            let secs = v.timestamp.elapsed().as_secs();

            #[cfg(debug_assertions)]
            if secs >= CLEANUP_TIMEOUT {
                println!("Cleaning up {k}");
            }

            secs < CLEANUP_TIMEOUT
        });
    }
}
