```rust
use tauri::Manager;

pub fn compile_ios() -> Result<(), String> {
    let config = include_str!("../../config.toml");
    let tauri_config = tauri::Config::from_string(config).unwrap();

    let context = tauri::generate_context!(tauri_config, "../../").unwrap();

    match tauri::Builder::from_context(context)
        .setup(|_app| {
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![handle_message])
        .run(tauri::generate_context!(tauri_config, "../../").unwrap()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to compile for iOS: {}", e)),
    }
}

#[tauri::command]
fn handle_message(window: tauri::Window, message: String) {
    window.emit("messageSent", message).unwrap();
}
```