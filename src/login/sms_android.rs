use tauri::api::dialog::message;
use tauri::api::sms::send;

pub struct SmsAndroidLogin {
    username: String,
    password: String,
}

impl SmsAndroidLogin {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn login(&self) -> Result<(), String> {
        if self.username.is_empty() || self.password.is_empty() {
            return Err("Username or password cannot be empty".to_string());
        }

        // Here you would normally call the Android SMS API to authenticate the user
        // For the sake of this example, we'll just simulate a successful login
        let login_successful = true;

        if login_successful {
            super::loggedInUser = Some(self.username.clone());
            message::info("Login successful", "You are now logged in");
            Ok(())
        } else {
            Err("Login failed".to_string())
        }
    }
}
