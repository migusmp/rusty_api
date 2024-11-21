use crate::models::chat::ChatState;
use crate::models::user::Payload;
use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{Path, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};

pub async fn create_chat(
    Path(room_id): Path<String>,
    Extension(state): Extension<Arc<Mutex<ChatState>>>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.create_room(room_id);
    "Room created".into_response()
}

pub async fn join_chat(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    Extension(state): Extension<Arc<Mutex<ChatState>>>,
    Extension(payload): Extension<Payload>,
) -> impl IntoResponse {
    println!("Usuario conectado: {}", payload.name);
    ws.on_upgrade(move |socket| handle_socket(socket, room_id, state, payload))
}

async fn handle_socket(
    socket: WebSocket,
    room_id: String,
    state: Arc<Mutex<ChatState>>,
    user: Payload,
) {
    let (mut sender, mut receiver) = socket.split();

    let user_channel = {
        let mut state = state.lock().unwrap();
        state.join_room(&room_id, user.name.clone())
    };

    let mut user_channel_rx = user_channel.subscribe();
    let tx_to_client = tokio::spawn(async move {
        while let Ok(msg) = user_channel_rx.recv().await {
            if let Err(e) = sender.send(msg).await {
                eprintln!("Error sending message: {}", e);
                break;
            }
        }
    });

    // Manejar mensajes recibidos desde el cliente WebSocket
    while let Some(Ok(msg)) = receiver.next().await {
        let message_content = match msg {
            Message::Text(text) => text.clone(), // Extrae el texto del mensaje
            Message::Binary(_) => "[binary data]".to_string(), // Mensajes binarios no soportados
            _ => "[unsupported message type]".to_string(), // Otros tipos no soportados
        };

        let message = Message::Text(format!("{}: {}", user.name, message_content));
        let state = state.lock().unwrap();

        // Reenviar mensaje a todos los usuarios de la sala
        if let Some(room) = state.rooms.get(&room_id) {
            for (user_id, sender) in &room.users {
                if user_id != &user.name {
                    if let Err(e) = sender.send(message.clone()) {
                        eprintln!("Error enviando mensaje a {}: {}", user_id, e);
                    }
                }
            }
        }
    }

    // Limpiar al usuario cuando se desconecte
    {
        let mut state = state.lock().unwrap();
        if let Some(room) = state.rooms.get_mut(&room_id) {
            room.users.remove(&user.name);
            println!("Usuario {} se desconectó de la sala {}", user.name, room_id);

            // Eliminar la sala si está vacía
            if room.users.is_empty() {
                state.rooms.remove(&room_id);
                println!("Sala {} eliminada por estar vacía", room_id);
            }
        }
    }

    // Esperar a que termine la tarea de envío al cliente
    let _ = tx_to_client.await;
}
