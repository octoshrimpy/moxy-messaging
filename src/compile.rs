use tauri::Builder;

pub fn compile_app(currentPlatform: &str) -> Result<(), String> {
    match currentPlatform {
        "android" => {
            Builder::default()
                .run(|app| {
                    Ok(())
                })
                .map_err(|err| err.to_string())
        },
        "ios" => {
            Builder::default()
                .run(|app| {
                    Ok(())
                })
                .map_err(|err| err.to_string())
        },
        _ => Err(String::from("Invalid platform")),
    }
}