use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .arg("check")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        panic!("Compilation errors detected. Please fix them before proceeding.");
    }

    println!("No compilation errors detected. Proceeding with the build...");
}