use crate::models::chat::ChatState;
use axum::extract::ws::WebSocket;
use axum::{
    extract::{Path, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use std::sync::{Arc, Mutex};

pub async fn create_chat(
    Path(room_id): Path<String>,
    Extension(state): Extension<Arc<Mutex<ChatState>>>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.create_room(room_id);
    println!("Rooms: {:?}", state);
    "Room created".into_response()
}

pub async fn join_chat(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    Extension(state): Extension<Arc<Mutex<ChatState>>>,
) -> impl IntoResponse {
    println!("accion de actualizar a ws");
    ws.on_upgrade(move |socket| handle_socket(socket, room_id, state))
}

async fn handle_socket(socket: WebSocket, _room_id: String, _state: Arc<Mutex<ChatState>>) {
    println!("socket: {:?}", socket)
}
