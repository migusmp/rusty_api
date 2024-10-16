use std::collections::HashMap;

use tokio::sync::broadcast;

pub struct ChatState {
    rooms: HashMap<String, broadcast::Sender<String>>, // Mapa de salas donde la clave es el room_id y el valor es un canal broadcast
}
impl Default for ChatState {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    // Crear una nueva sala.
    pub fn create_room(&mut self, room_id: String) {
        let (tx, _rx) = broadcast::channel(100);
        self.rooms.insert(room_id, tx);
    }

    // Obtener el canal broadcast de una sala.
    pub fn get_room(&self, room_id: &String) -> Option<broadcast::Sender<String>> {
        self.rooms.get(room_id).cloned()
    }
}
