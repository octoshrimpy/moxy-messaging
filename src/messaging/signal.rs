```rust
use tauri::api::dialog::message;
use tauri::api::http::Response;
use tauri::api::http::StatusCode;

pub struct SignalMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: i64,
}

impl SignalMessage {
    pub fn new(sender: String, content: String, timestamp: i64) -> Self {
        Self {
            sender,
            content,
            timestamp,
        }
    }
}

pub async fn send_signal_message(message: SignalMessage) -> Result<Response, String> {
    let url = "https://api.signal.org/v1/messages";
    let response = tauri::api::http::post(url, Some(message), None).await;

    match response {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                message::info("Message Sent", "Your message was sent successfully.");
                Ok(response)
            } else {
                Err("Failed to send message".into())
            }
        }
        Err(_) => Err("Failed to send message".into()),
    }
}
```