```rust
use std::process::Command;
use std::io::{self, Write};

pub fn run_command(command: &str) -> io::Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}

pub fn check_rust_errors() -> io::Result<()> {
    run_command("cargo check")
}

pub fn check_tauri_errors() -> io::Result<()> {
    run_command("tauri info")
}
```