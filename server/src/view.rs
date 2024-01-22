use axum::{Router, http::StatusCode};
use tower_http::services::ServeDir;

pub fn serve_view_dir() -> Router {
    Router::new()
    .route_service("/view", serve_dir_service())
    // .fallback(not_found)
}

pub fn serve_dir_service() -> ServeDir {
    ServeDir::new("wwwroot/view")
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
