use tauri::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct IrcLogin {
    username: String,
    password: String,
}

#[tauri::command]
fn irc_login(user: IrcLogin) -> Result<(), String> {
    // Here we would normally interact with the IRC API to log the user in
    // For the sake of this example, we'll just print the username and password
    println!("Logging in with username: {} and password: {}", user.username, user.password);

    // If login was successful, we would return Ok(())
    // If there was an error, we would return Err(error_message)
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![irc_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
