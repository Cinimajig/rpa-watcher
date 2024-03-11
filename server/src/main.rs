mod api;
mod rpa_state;

use axum::{
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    Router,
};
use std::env;
use tower_http::services::ServeDir;

const DEFAULT_PORT: u16 = 80;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = match env::var("HTTP_PLATFORM_PORT").or(env::var("ASPNETCORE_PORT")) {
        Ok(port) => port.parse().unwrap_or(DEFAULT_PORT),
        _ => DEFAULT_PORT,
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
        .nest("/api", api::router());

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;

    let cleanup_job = tokio::spawn(api::cleanup_timer());
    axum::serve(listener, app).await?;

    Ok(cleanup_job.await?)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    let s = format!("404 - No route found for {uri}");
    println!("{}", &s);
    (StatusCode::NOT_FOUND, s)
}
