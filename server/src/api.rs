use std::{collections::{HashMap, VecDeque}, sync::Arc};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use rpa::RpaData;
use tokio::{sync::RwLock, time::Instant};


pub const CLEANUP_TIMER_INTERVAL: u64 = 5;
pub const CLEANUP_TIMEOUT: u64 = 32;
pub const DEFAULT_SIZE: usize = 10;
pub const HISTORY_LIMIT: usize = 50;

pub type RpaItems = HashMap<String, RpaValue>;

#[derive(PartialEq, Clone)]
pub struct RpaValue {
    pub timestamp: Instant,
    pub data: RpaData,
}

impl RpaValue {
    pub fn new(timestamp: Instant, data: RpaData) -> Self {
        Self { timestamp, data }
    }
}

#[derive(Clone)]
pub struct GlobalState {
    pub kill_flag: bool,
    pub prdb: Option<Arc<RwLock<crate::db::Database>>>,
    pub paapi: Option<Arc<RwLock<crate::pa_api::PowerAutomateAPI>>>,
    pub rpa: Arc<RwLock<RpaItems>>,
    pub failed_rpa: Arc<RwLock<RpaItems>>,
    pub history_rpa: Arc<RwLock<VecDeque<RpaData>>>,
}

pub fn router(database: GlobalState) -> Router {
    Router::new()
        .route("/getrpa", get(get_rpadata))
        .route("/getfailed", get(get_failed_rpadata))
        .route("/gethistory", get(get_history_rpadata))
        .route("/checkin", post(post_checkin))
        .with_state(database)
        .fallback(crate::fallback)
}

async fn get_failed_rpadata(// headers: HeaderMap,
    State(state): State<GlobalState>,
) -> Json<Vec<RpaData>> {
    let data = state.failed_rpa.read().await;
    Json(data.iter().map(|(_k, v)| v.data.clone()).collect())
}

async fn get_rpadata(// headers: HeaderMap,
    State(state): State<GlobalState>,
) -> Json<Vec<RpaData>> {
    let data = state.rpa.read().await;

    #[cfg(debug_assertions)]
    println!("Sending {} items", data.len());

    Json(data.iter().map(|(_k, v)| v.data.clone()).collect())
}

async fn get_history_rpadata(// headers: HeaderMap,
    State(state): State<GlobalState>,
) -> Json<VecDeque<RpaData>> {
    let history = state.history_rpa.read().await;

    #[cfg(debug_assertions)]
    println!("Sending {} items", history.len());

    Json(history.clone())
}

async fn post_checkin(
    // headers: HeaderMap,
    State(state): State<GlobalState>,
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

    let mut data = state.rpa.write().await;
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
                    if let Some(paapi) = state.paapi.clone() {
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
                                    eprintln!("Failed to find Power Automate flow. {err}");
                                }
                            }
                        }
                    }
                }
                rpa::RpaEngine::ProcessRobot => {
                    // Search the PR database for a name.
                    if let Some(db_client) = state.prdb.clone() {
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

pub async fn cleanup_timer(state: GlobalState) {
    use tokio::time::*;

    loop {
        if state.kill_flag {
            break;
        }

        // Waits for CLEANUP_TIMER_INTERVAL seconds to not keeping it 
        // locked all the time.
        sleep(Duration::from_secs(CLEANUP_TIMER_INTERVAL)).await;

        // Retrieves a lock for the running and history.
        let mut data = state.rpa.write().await;
        let mut history = state.history_rpa.write().await;
        
        // Collects a copy of alle the items to remove.
        let removed: Vec<(String, RpaValue)> = data.iter().filter(|(k, v)| {
            let secs = v.timestamp.elapsed().as_secs();

            #[cfg(debug_assertions)]
            if secs >= CLEANUP_TIMEOUT {
                println!("Cleaning up {k}");
            }

            secs > CLEANUP_TIMEOUT
        }).map(|(k, v)| (k.clone(), v.clone())).collect();

        // Adds it to the history and removing from running.
        removed.into_iter().for_each(|(k, v)| {
            history.push_front(v.data);
            data.remove(&k);
        });

        // Release the lock.
        std::mem::drop(data);

        while history.len() > HISTORY_LIMIT {
            history.pop_back();
        }
    }
}
