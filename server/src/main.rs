mod api;
mod config;
mod db;
mod pa_api;

use axum::{
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    Router,
};
use std::{
    collections::{HashMap, VecDeque},
    env,
    sync::Arc,
};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    for (name, val) in env::vars() {
        println!("{name}={val}");
    }

    let token = env::var("RW_TOKEN").unwrap_or_default();

    let config::PRConfig {
        http_port,
        db_conn_str,
    } = config::PRConfig::load();

    // ProcessRobot (refactor?).
    let prdb = match db_conn_str {
        Some(db_conn_str) => match db::create(&db_conn_str).await {
            Ok(db) => Some(db),
            Err(err) => {
                eprintln!("Error connectiong to ProcessRobot database. {err}");
                None
            }
        },
        None => None,
    };

    // Power Automate/MS Dynamics.
    let paapi = match pa_api::PowerAutomateAPI::load() {
        Ok(pa) => Some(Arc::new(RwLock::new(pa))),
        Err(err) => {
            eprintln!("Error connectiong to Power Automate API. {err}");
            None
        }
    };

    // Global application state.
    // This is share with each api request and the cleanup routine.
    let global_state = api::GlobalState {
        token: Arc::new(token.into()),
        kill_flag: Arc::new(RwLock::new(false)),
        prdb, paapi,
        rpa: Arc::new(RwLock::new(HashMap::with_capacity(api::DEFAULT_SIZE))),
        failed_rpa: Arc::new(RwLock::new(HashMap::with_capacity(api::DEFAULT_SIZE))),
        history_rpa: Arc::new(RwLock::new(VecDeque::with_capacity(api::HISTORY_LIMIT))),
    };

    let app = Router::new()
        // .nest_service("/", ServeDir::new("wwwroot").not_found_service(fallback.into_service()))
        .fallback_service(ServeDir::new("wwwroot").not_found_service(fallback.into_service()))
        .nest("/api", api::router(global_state.clone()));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", http_port)).await?;
    let cleanup_job = tokio::spawn(api::cleanup_timer(global_state.clone()));

    axum::serve(listener, app).await?;

    *global_state.kill_flag.write().await = true;
    Ok(cleanup_job.await?)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    let s = format!("404 - No route found for {uri}");
    println!("{}", &s);
    (StatusCode::NOT_FOUND, s)
}
