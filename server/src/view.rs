use axum::{Router, http::StatusCode};

pub fn serve_view_dir() -> Router {
    use tower_http::services::ServeDir;

    Router::new()
    .nest_service("/view", ServeDir::new("wwwroot"))
    .fallback(not_found)
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
