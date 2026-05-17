pub mod api;
pub mod clipboard;
pub mod config;
pub mod hotkey;
pub mod instructions;
pub mod tray;

use std::fs;

pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    log::info!("Starting GhostWriter v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let config_dir = config::config_dir();

            if let Err(e) = fs::create_dir_all(&config_dir) {
                log::error!("Failed to create config directory: {}", e);
            }

            if let Err(e) = instructions::ensure_default_instruction(&config_dir) {
                log::error!("Failed to create default instruction: {}", e);
            }

            if let Err(e) = instructions::create_sample_instructions(&config_dir) {
                log::error!("Failed to create sample instructions: {}", e);
            }

            let _settings = match config::load_settings() {
                Ok(s) => {
                    log::info!("Loaded settings: model={}", s.model);
                    s
                }
                Err(e) => {
                    log::warn!("Failed to load settings, using defaults: {}", e);
                    let default = config::Settings::default();
                    if let Err(e) = config::save_settings(&default) {
                        log::error!("Failed to save default settings: {}", e);
                    }
                    default
                }
            };

            if let Err(e) = hotkey::init(app.handle()) {
                log::error!("Failed to initialize hotkey: {}", e);
            }

            if let Err(e) = tray::create_tray(app) {
                log::error!("Failed to create tray: {}", e);
            }

            log::info!("GhostWriter initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            log::error!("GhostWriter fatal error: {}", e);
            std::process::exit(1);
        });
}
