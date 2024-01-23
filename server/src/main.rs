mod api;
mod state;
mod view;

use axum::{
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    routing::get,
    Router, ServiceExt,
};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .nest_service(
            "/",
            ServeDir::new("wwwroot")
            .not_found_service(fallback.into_service()),
        )
        .nest("/api", api::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    let s = format!("No route found for {uri}");
    println!("{}", &s);
    (StatusCode::NOT_FOUND, s)
}
