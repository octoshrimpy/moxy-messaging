```rust
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub debug: bool,
    pub log_level: String,
    pub error_file: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Path::new("config.json");
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;

        Ok(config)
    }
}
```