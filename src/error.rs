```rust
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Tauri(tauri::Error),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Tauri(err) => write!(f, "Tauri error: {}", err),
            AppError::Other(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<tauri::Error> for AppError {
    fn from(err: tauri::Error) -> AppError {
        AppError::Tauri(err)
    }
}
```