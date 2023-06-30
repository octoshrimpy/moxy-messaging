use tauri::bundle::{BundleBinary, AndroidAppBundle};

pub fn compile_android() -> Result<(), String> {
    let settings = tauri::Settings {
        debug: false,
        config: tauri::Config {
            tauri: tauri::TauriConfig {
                bundle: tauri::BundleConfig {
                    active: true,
                    targets: vec![tauri::bundle::Target::Android],
                    android: Some(tauri::bundle::AndroidConfig {
                        package_name: "com.example.app".into(),
                        package_label: "Example App".into(),
                        icon: Some("src/assets/logo.png".into()),
                        use_work_profile: false,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
    };

    let context = tauri::generate_context!(settings).unwrap();
    let binary = BundleBinary::from_path(&context, "src/target/release/app").unwrap();
    let bundle = AndroidAppBundle::new(&context, binary);
    match bundle.build() {
        Ok(_) => {
            println!("Android app bundle built successfully.");
            Ok(())
        }
        Err(e) => Err(format!("Failed to build Android app bundle: {}", e)),
    }
}