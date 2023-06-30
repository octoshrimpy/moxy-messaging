```rust
use std::error::Error;
use tauri::AppBuilder;
use crate::config::Config;
use crate::error::SetupError;

pub fn setup_app() -> Result<AppBuilder, Box<dyn Error>> {
    let config = Config::load()?;

    let app = tauri::AppBuilder::new()
        .setup(move |_app| {
            // setup code here
            Ok(())
        })
        .build(tauri::generate_context!())
        .map_err(|err| SetupError::new(err))?;

    Ok(app)
}
```