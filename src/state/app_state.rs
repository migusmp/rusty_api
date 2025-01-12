use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, mpsc, Mutex};

pub struct AppState {
    pub global_broadcast: broadcast::Sender<String>,

    pub friend_notifications: Arc<Mutex<HashMap<i32, mpsc::Sender<String>>>>,

    pub config: AppConfig,
}

pub struct AppConfig {
    pub app_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: String::from("Migus App"),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        let (global_broadcast, _) = broadcast::channel::<String>(1000);
        Self {
            global_broadcast,
            friend_notifications: Arc::new(Mutex::new(HashMap::new())),
            config: AppConfig::default(),
        }
    }

    // Método para añadir un usuario conectado
    pub async fn add_user_to_friend_notifications(&self, user_id: i32) -> mpsc::Receiver<String> {
        let (sender, receiver) = mpsc::channel::<String>(100); // Crear canal para el usuario
        let mut notifications = self.friend_notifications.lock().await;
        notifications.insert(user_id, sender); // Agregar el canal a la lista de usuarios conectados
        receiver // Devolver el receptor para que el usuario pueda recibir mensajes
    }

    pub async fn send_friend_notification(
        &self,
        friend_id: i32,
        user_name: &String,
    ) -> Result<(), String> {
        let notifications = self.friend_notifications.lock().await;
        let message = format!("{} send to you a friend request!", user_name);

        if let Some(sender) = notifications.get(&friend_id) {
            match sender.try_send(message) {
                Ok(_) => Ok(()),
                Err(_e) => Err(format!("No se pudo enviar la notificacion a {}", friend_id)),
            }
        } else {
            Err(format!("El usuario {} no esta conectado", friend_id))
        }
    }

    pub fn add_user_to_global_broadcast(&self) -> broadcast::Receiver<String> {
        // Crea un nuevo receptor para el broadcast
        let (_tx, rx) = broadcast::channel::<String>(100); // Buffer de 100 mensajes

        rx // Devolver el receptor para que el usuario pueda escuchar los mensajes
    }

    // Método para enviar un mensaje global a todos los usuarios
    pub fn send_global_broadcast(&self, message: String) {
        let _ = self.global_broadcast.send(message); // Enviar el mensaje al canal global
    }
}
