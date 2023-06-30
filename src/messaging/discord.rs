```rust
use tauri::api::dialog::message;
use crate::User;
use crate::Message;

pub fn send_discord_message(user: &User, message: &Message) -> Result<(), String> {
    if user.discord_token.is_none() {
        return Err("User is not logged in to Discord".to_string());
    }

    let discord_token = user.discord_token.as_ref().unwrap();

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://discord.com/api/channels/{channel_id}/messages")
        .bearer_auth(discord_token)
        .json(&message)
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                message::info("Message Sent", "Your message was sent successfully on Discord.");
                Ok(())
            } else {
                Err("Failed to send message on Discord".to_string())
            }
        },
        Err(_) => Err("Failed to send message on Discord".to_string()),
    }
}
```