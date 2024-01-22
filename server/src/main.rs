mod api;
mod view;
mod state;

use axum::{routing::get, Router, http::StatusCode, response::Redirect};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(redirect))
        .nest_service("/view", view::serve_dir_service())
        .nest("/api", api::router());

    // let view_app = view::serve_view_dir();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
    axum::serve(listener, app).await?;

    // let api_listener = tokio::spawn(async move {
    //     axum::serve(tokio::net::TcpListener::bind("0.0.0.0:1997").await.unwrap(), api_app).await
    // });
    // let view_listener = tokio::spawn(async move {
    //     axum::serve(
    //         tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap(), view_app
    //     ).await
    // });

    // let (api, view) = tokio::join!(api_listener, view_listener);
    // assert_eq!(true, api.and(view).is_ok());

    Ok(())
}

// basic handler that responds with a static string
async fn redirect() -> Redirect {
    Redirect::permanent("/view")
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
