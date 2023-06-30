```rust
use tauri::Manager;

pub struct IrcMessage {
    pub sender: String,
    pub content: String,
}

pub async fn send_irc_message(manager: &Manager, message: IrcMessage) -> Result<(), String> {
    let current_message = format!("{}: {}", message.sender, message.content);
    manager.emit("messageSent", current_message).unwrap();
    Ok(())
}

pub async fn receive_irc_message(manager: &Manager) -> Result<IrcMessage, String> {
    let message = IrcMessage {
        sender: String::from("IRC User"),
        content: String::from("Hello from IRC!"),
    };
    Ok(message)
}
```