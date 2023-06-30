use tauri::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct DiscordLogin {
    username: String,
    password: String,
}

#[tauri::command]
async fn login_discord(login: DiscordLogin) -> Result<(), String> {
    // Here we would use the Discord API to log in the user
    // This is a placeholder as the actual implementation would require handling of OAuth and other security measures
    let discord_api = "https://discord.com/api";
    let response = reqwest::post(format!("{}/login", discord_api))
        .json(&login)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let user: User = response.json().await.unwrap();
                tauri::event::emit("loginSuccess", Some(user)).unwrap();
                Ok(())
            } else {
                Err("Failed to log in to Discord".into())
            }
        }
        Err(_) => Err("Failed to connect to Discord".into()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login_discord])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
