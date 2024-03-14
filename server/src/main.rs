mod api;
mod config;
mod db;
mod rpa_state;

use axum::{
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    Router,
};
use std::env;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config::Config {
        http_port,
        db_conn_str,
    } = config::Config::load();

    let database = match db_conn_str {
        Some(db_conn_str) => {
            match db::create(&db_conn_str).await {
                Ok(db) => Some(db),
                Err(err) => {
                    eprintln!("Error connectiong to ProcessRobot database. {err}");
                    None
                }
            }
        },
        None => None,
    };

    #[cfg(debug_assertions)]
    for (name, val) in env::vars() {
        println!("{name}={val}");
    }

    let app = Router::new()
        .nest_service(
            "/",
            ServeDir::new("wwwroot").not_found_service(fallback.into_service()),
        )
        .nest("/api", api::router(database));

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
