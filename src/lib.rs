```rust
extern crate tauri;

mod error;
mod utils;
mod config;
mod app;
mod commands;
mod events;
mod setup;
mod build;
mod tests;

pub use error::Error;
pub use utils::Utils;
pub use config::Config;
pub use app::App;
pub use commands::Commands;
pub use events::Events;
pub use setup::Setup;
pub use build::Build;
pub use tests::Tests;

use tauri::Manager;

pub fn run() -> Result<(), Error> {
    let config = Config::new()?;
    let app = App::new(&config)?;
    let commands = Commands::new(&app)?;
    let events = Events::new(&app)?;
    let setup = Setup::new(&app)?;
    let build = Build::new(&app)?;
    let tests = Tests::new(&app)?;

    setup.run()?;
    build.run()?;
    tests.run()?;

    Manager::new().mount(app).manage(commands).manage(events).run(|app_handle, e| match e {
        _ => app_handle.exit(0),
    })
}
```