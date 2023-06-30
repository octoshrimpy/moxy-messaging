```rust
use tauri::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct SendMessagePayload {
    user: String,
    message: String,
}

#[tauri::command]
fn send_matrix_message(payload: SendMessagePayload) -> Result<(), String> {
    let user = payload.user;
    let message = payload.message;

    // Here we would have the logic to send a message via Matrix
    // This is a placeholder as the actual implementation would require interaction with Matrix's API
    println!("Message sent to Matrix by {}: {}", user, message);

    Ok(())
}

pub fn setup(app: &mut tauri::Builder) {
    app.command("sendMatrixMessage", send_matrix_message);
}
```