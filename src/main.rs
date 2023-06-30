```rust
extern crate tauri;

mod app;
mod login;
mod messaging;
mod compile;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```