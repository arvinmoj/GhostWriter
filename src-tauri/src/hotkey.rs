use std::sync::atomic::{AtomicBool, Ordering};
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

static PROCESSING: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "macos")]
const DEFAULT_SHORTCUT: &str = "cmd+shift+r";

#[cfg(target_os = "windows")]
const DEFAULT_SHORTCUT: &str = "ctrl+shift+r";

#[cfg(target_os = "linux")]
const DEFAULT_SHORTCUT: &str = "ctrl+shift+r";

pub fn init(app: &AppHandle) -> Result<(), String> {
    register_hotkey(app, DEFAULT_SHORTCUT)?;
    log::info!("Global hotkey registered: {}", DEFAULT_SHORTCUT);
    Ok(())
}

pub fn register_hotkey(app: &AppHandle, shortcut: &str) -> Result<(), String> {
    let parsed: Shortcut = shortcut.parse()
        .map_err(|e| format!("Failed to parse shortcut: {}", e))?;

    app.global_shortcut().on_shortcut(parsed, move |_app, _shortcut, event| {
        if event.state == ShortcutState::Pressed {
            if !PROCESSING.swap(true, Ordering::SeqCst) {
                std::thread::spawn(move || {
                    if let Err(e) = process_text() {
                        log::error!("Text processing failed: {}", e);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    PROCESSING.store(false, Ordering::SeqCst);
                });
            }
        }
    }).map_err(|e| format!("Failed to register hotkey: {}", e))?;

    Ok(())
}
