mod api;
mod config;
mod db;
mod pa_api;
mod rpa_state;

use axum::{
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    Router,
};
use tokio::sync::RwLock;
use std::{env, sync::Arc};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    for (name, val) in env::vars() {
        println!("{name}={val}");
    }

    let config::PRConfig {
        http_port,
        db_conn_str,
        ..
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

    // Power Autmate/MS Dynamics.
    let paapi = match pa_api::PowerAutomateAPI::load() {
        Ok(pa) => Some(Arc::new(RwLock::new(pa))),
        Err(err) => {
            eprintln!("Error connectiong to ProcessRobot database. {err}");
            None
        }
    };

    let app = Router::new()
        .nest_service(
            "/",
            ServeDir::new("wwwroot").not_found_service(fallback.into_service()),
        )
        
        .nest("/api", api::router(api::GLobalState { prdb, paapi }));
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", http_port)).await?;

    let cleanup_job = tokio::spawn(api::cleanup_timer());
    axum::serve(listener, app).await?;

    Ok(cleanup_job.await?)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    let s = format!("404 - No route found for {uri}");
    println!("{}", &s);
    (StatusCode::NOT_FOUND, s)
}
