use tauri::Window;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MatrixLogin {
    username: String,
    password: String,
}

pub async fn login(window: Window, login_data: MatrixLogin) -> Result<(), String> {
    // Here we would normally interact with the Matrix API to log the user in.
    // For the sake of this example, we'll just simulate a successful login.
    let user = User {
        username: login_data.username,
        platform: "Matrix".to_string(),
    };

    window.emit("loginSuccess", user).unwrap();

    Ok(())
}
