use crate::User;

pub mod signal;
pub mod discord;
pub mod irc;
pub mod matrix;
pub mod sms_android;

pub struct Login {
    user: Option<User>,
}

impl Login {
    pub fn new() -> Self {
        Login { user: None }
    }

    pub fn login_signal(&mut self, username: String, password: String) {
        self.user = Some(signal::login(username, password));
    }

    pub fn login_discord(&mut self, username: String, password: String) {
        self.user = Some(discord::login(username, password));
    }

    pub fn login_irc(&mut self, username: String, password: String) {
        self.user = Some(irc::login(username, password));
    }

    pub fn login_matrix(&mut self, username: String, password: String) {
        self.user = Some(matrix::login(username, password));
    }

    pub fn login_sms_android(&mut self, username: String, password: String) {
        self.user = Some(sms_android::login(username, password));
    }

    pub fn get_logged_in_user(&self) -> Option<&User> {
        self.user.as_ref()
    }
}
