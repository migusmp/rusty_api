use crate::models::chat::ChatState;
use axum::{extract::Path, response::IntoResponse, Extension};
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
