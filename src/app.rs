```rust
use tauri::AppBuilder;
use crate::error::AppError;
use crate::config::AppConfig;
use crate::commands::register_commands;
use crate::events::register_events;
use crate::setup::setup_app;

pub struct Application {
    config: AppConfig,
}

impl Application {
    pub fn new() -> Result<Self, AppError> {
        let config = AppConfig::load()?;
        Ok(Self { config })
    }

    pub fn run(&self) -> Result<(), AppError> {
        let mut app = AppBuilder::new().setup(|app| {
            setup_app(app, &self.config)
        });

        register_commands(&mut app)?;
        register_events(&mut app)?;

        app.run().map_err(|e| AppError::TauriError(e))
    }
}
```