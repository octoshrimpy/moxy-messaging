```rust
use crate::User;
use crate::loggedInUser;

pub fn login(username: String, password: String) -> Result<User, &'static str> {
    // Here we would normally call the Signal API to authenticate the user.
    // For the sake of this example, we'll just simulate a successful login.
    if username == "test" && password == "test" {
        let user = User {
            username: username,
            platform: "Signal".to_string(),
        };
        loggedInUser = Some(user.clone());
        Ok(user)
    } else {
        Err("Invalid username or password")
    }
}
```