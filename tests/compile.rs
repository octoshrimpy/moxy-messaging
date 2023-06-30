```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tauri::AppBuilder;

    #[test]
    fn test_compile_android() {
        let app = AppBuilder::new().setup(|app| {
            let currentPlatform = "android";
            compileApp(app, currentPlatform);
        }).build().unwrap();

        assert_eq!(app.run(), Ok(()));
    }

    #[test]
    fn test_compile_ios() {
        let app = AppBuilder::new().setup(|app| {
            let currentPlatform = "ios";
            compileApp(app, currentPlatform);
        }).build().unwrap();

        assert_eq!(app.run(), Ok(()));
    }
}
```