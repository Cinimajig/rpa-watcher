use crate::state::AppState;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use rpa::RpaData;
use serde::Serialize;

type ApiState = AppState<Vec<RpaData>>;

pub fn router() -> Router {
    let buffer_data: ApiState = AppState::new(Vec::with_capacity(10));

    Router::new()
        // .route("/listener", get(ws_handler))
        .route("/getrpa", get(get_rpadata))
        .route("/checkin", post(post_checkin))
        .fallback(not_found)
        .with_state(buffer_data)
}

/// TODO!
// async fn _ws_handler(
//     ws: WebSocketUpgrade,
//     _headers: HeaderMap,
//     ConnectInfo(addr): ConnectInfo<SocketAddr>,
// ) -> impl IntoResponse {
//     // match headers.get("Api-Token") {
//     //     Some(s) if s => s,
//     //     None => todo!(),
//     // }
//     ws.on_upgrade(move |socket| _handle_socket(socket, addr))
// }

// /// Sample - https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs
// async fn _handle_socket(mut _socket: WebSocket, _who: SocketAddr) {

// }

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn get_rpadata(headers: HeaderMap, State(state): State<ApiState>) -> Json<Vec<RpaData>> {
    println!("headers: {headers:?}");

    let data = state.data.lock().await;
    Json(data.clone())
}

async fn post_checkin(
    headers: HeaderMap,
    State(state): State<ApiState>,
    Json(payload): Json<Vec<RpaData>>,
) -> StatusCode {
    let mut data = state.data.lock().await;
    payload.into_iter().for_each(|item| data.push(item));
    
    StatusCode::OK
}
