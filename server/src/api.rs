use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use rpa::RpaData;

pub fn router() -> Router {
    let buffer_data: Vec<RpaData> = Vec::with_capacity(10);

    Router::new()
        .route("/getrpa", get(get_rpadata))
        .route("/checkin", post(post_checkin))
        .with_state(buffer_data)
}

pub async fn get_rpadata(headers: HeaderMap, state: State<Vec<RpaData>>) -> Json<Vec<RpaData>> {
    todo!()
}

pub async fn post_checkin(headers: HeaderMap, Json(payload): Json<Vec<RpaData>>) -> StatusCode {
    todo!()
}
