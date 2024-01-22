mod api;
mod view;
mod state;

use axum::{routing::get, Router, http::StatusCode, response::Redirect};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_app = Router::new()
        .route("/", get(redirect))
        .fallback(not_found)
        .nest("/api", api::router());

    let view_app = view::serve_view_dir();

    let api_listener = tokio::spawn(async move {
        axum::serve(tokio::net::TcpListener::bind("0.0.0.0:1997").await.unwrap(), api_app).await
    });
    let view_listener = tokio::spawn(async move {
        axum::serve(
            tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap(), view_app
        ).await
    });

    let (api, view) = tokio::join!(api_listener, view_listener);
    api.unwrap();
    view.unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn redirect() -> Redirect {
    Redirect::to("/view")
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
