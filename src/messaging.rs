use tauri::Manager;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    sender: String,
    content: String,
    timestamp: String,
}

pub async fn send_message(window: tauri::Window, message: Message) -> Result<(), String> {
    window.emit("messageSent", message).map_err(|err| err.to_string())
}

pub async fn get_current_message(window: tauri::Window) -> Result<Message, String> {
    window.listen("currentMessage").await.map_err(|err| err.to_string())
}
