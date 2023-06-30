```rust
use tauri::Manager;
use crate::login::login;
use crate::messaging::sendMessage;
use crate::compile::compileApp;

#[derive(Default)]
struct AppState {
    loggedInUser: Option<User>,
    currentPlatform: Option<String>,
    currentMessage: Option<Message>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![login, sendMessage, compileApp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```