use crate::models::chat::ChatState;
use crate::models::user::Payload;
use axum::extract::ws::{Message, WebSocket};

use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn handle_socket_for_stats(
    socket: WebSocket,
    room_id: String,
    state: Arc<RwLock<ChatState>>,
    _user: Payload,
) {
    let (mut sender, mut receiver) = socket.split();

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Enviar estadísticas periódicas, por ejemplo:
                let state = state.read().await;
                if let Some(user_count) = state.get_room_user_count(&room_id) {
                    let msg = Message::Text(format!("{}", user_count));
                    // Verificar si la conexión está abierta
                    if sender.send(msg).await.is_err() {
                        eprintln!("No se pudo enviar el mensaje, WebSocket cerrado o error en la conexión");
                        break;
                    }
                }
            },
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        println!("Mensaje recibido: {}", text);
                    },
                    Some(Ok(Message::Binary(data))) => {
                        println!("Mensaje binario recibido: {:?}", data);
                    },
                    Some(Ok(Message::Pong(_))) => {
                        println!("Pong recibido");
                    },
                    Some(Ok(Message::Ping(_))) => {
                        println!("Ping recibido");
                    }
                    Some(Ok(Message::Close(reason))) => {
                        if let Some(reason) = reason {
                            println!("Conexión cerrada: {:?}", reason);
                        } else {
                            println!("Conexión cerrada sin razón");
                        }
                        break; // Salir si la conexión se cierra
                    },
                    Some(Err(e)) => {
                        eprintln!("Error en la recepción del mensaje: {}", e);
                        break; // Salir si hay error en la recepción
                    },
                    None => {
                        eprintln!("Conexión cerrada por el cliente");
                        break; // La conexión se ha cerrado
                    }
                }
            }
        }
    }

    // Aquí puedes agregar lógica adicional de limpieza si es necesario
    eprintln!("Fin de la conexión WebSocket");
}

pub async fn handle_socket(
    socket: WebSocket,
    room_id: String,
    state: Arc<RwLock<ChatState>>,
    user: Payload,
) {
    let (mut sender, mut receiver) = socket.split();

    let user_channel = {
        let mut state = state.write().await;
        state.join_room(&room_id, user.name.clone())
    };

    // Join message.
    let join_msg = Message::Text(format!("{} has joined the chat.", user.name));

    // Send join message.
    {
        let state = state.read().await;
        if let Some(room) = state.rooms.get(&room_id) {
            for (user_id, sender) in &room.users {
                if let Err(e) = sender.send(join_msg.clone()) {
                    eprintln!("Error enviando mensaje de bienvenida a {}: {}", user_id, e);
                }
            }
        }
    }

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
            Message::Text(text) => format!("{}: {}", user.name, text.clone()), // Extrae el texto del mensaje
            Message::Binary(_) => "[binary data]".to_string(), // Mensajes binarios no soportados
            _ => format!("{} left the chat.", user.name),
        };

        let message = Message::Text(format!("{}", message_content));

        let state = state.read().await;

        // Reenviar mensaje a todos los usuarios de la sala
        if let Some(room) = state.rooms.get(&room_id) {
            for (user_id, sender) in &room.users {
                if let Err(e) = sender.send(message.clone()) {
                    eprintln!("Error enviando mensaje a {}: {}", user_id, e);
                }
            }
        }
    }

    // Limpiar al usuario cuando se desconecte
    {
        let mut state = state.write().await;
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
