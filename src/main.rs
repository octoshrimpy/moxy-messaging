```rust
use tauri::Manager;
use std::process::Command;

mod error;
mod utils;
mod config;
mod app;
mod commands;
mod events;
mod setup;
mod build;
mod tests;

fn main() {
    let config = config::load_config().expect("Failed to load configuration");

    if let Err(e) = setup::setup_environment(&config) {
        error::log_error(&e);
        return;
    }

    let app_state = app::AppState::new(&config).expect("Failed to initialize application state");

    let mut app = tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![commands::handle_command, events::handle_event]);

    if let Err(e) = build::build_app(&mut app) {
        error::log_error(&e);
        return;
    }

    let mut cmd = Command::new("cargo");
    cmd.arg("check");
    let output = cmd.output().expect("Failed to execute command");

    if !output.status.success() {
        println!("Compilation error: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    app.run(tauri::generate_context!()).expect("Error while running Tauri application");
}
```