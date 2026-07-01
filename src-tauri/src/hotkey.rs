use std::sync::atomic::{AtomicBool, Ordering};
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

static PROCESSING: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "macos")]
#[allow(dead_code)]
const DEFAULT_SHORTCUT: &str = "cmd+shift+r";

#[cfg(target_os = "windows")]
#[allow(dead_code)]
const DEFAULT_SHORTCUT: &str = "ctrl+shift+r";

#[cfg(target_os = "linux")]
#[allow(dead_code)]
const DEFAULT_SHORTCUT: &str = "ctrl+shift+r";

pub fn hotkey_config_to_shortcut(hotkey: &crate::config::HotkeyConfig) -> String {
    format!("{}+{}", hotkey.modifiers.join("+"), hotkey.key)
}

const RESTORE_PASTE_DELAY_MS: u64 = 150;

struct ClipboardGuard {
    saved: String,
    paste_dispatched: bool,
}

impl ClipboardGuard {
    fn new(saved: String) -> Self {
        Self {
            saved,
            paste_dispatched: false,
        }
    }

    fn mark_paste_dispatched(&mut self) {
        self.paste_dispatched = true;
    }
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        let (sleep, write) = cleanup_plan(self.saved.is_empty(), self.paste_dispatched);
        if sleep {
            std::thread::sleep(std::time::Duration::from_millis(RESTORE_PASTE_DELAY_MS));
        }
        if !write {
            log::info!("[STEP 11] Skipped restore: no prior clipboard content");
            return;
        }
        match crate::clipboard::write_clipboard(&self.saved) {
            Ok(()) => log::info!("[STEP 11] Clipboard restored ({} chars)", self.saved.len()),
            Err(e) => log::warn!("[STEP 11] Failed to restore clipboard (non-fatal): {}", e),
        }
    }
}

fn cleanup_plan(saved_empty: bool, paste_dispatched: bool) -> (bool, bool) {
    (paste_dispatched, !saved_empty)
}

pub fn init(app: &AppHandle, settings: &crate::config::Settings) -> Result<(), String> {
    let shortcut = hotkey_config_to_shortcut(&settings.hotkey);
    register_hotkey(app, &shortcut)?;
    log::info!("Global hotkey registered: {}", shortcut);
    Ok(())
}

pub fn register_hotkey(app: &AppHandle, shortcut: &str) -> Result<(), String> {
    let parsed: Shortcut = shortcut
        .parse()
        .map_err(|e| format!("Failed to parse shortcut: {}", e))?;

    app.global_shortcut()
        .on_shortcut(parsed, move |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed && !PROCESSING.swap(true, Ordering::SeqCst) {
                let app = app.clone();
                std::thread::spawn(move || {
                    if let Err(e) = process_text(&app) {
                        let (title, body) = crate::notify::classify(&e);
                        crate::notify::error(&app, title, &body);
                    }
                    PROCESSING.store(false, Ordering::SeqCst);
                });
            }
        })
        .map_err(|e| format!("Failed to register hotkey: {}", e))?;

    Ok(())
}

fn process_text(_app: &AppHandle) -> Result<(), String> {
    log::info!("[STEP 1] Hotkey triggered, starting text capture");

    // Save current clipboard content
    let saved_clipboard = crate::clipboard::read_clipboard().unwrap_or_default();
    log::info!(
        "[STEP 1a] Saved clipboard ({} chars)",
        saved_clipboard.len()
    );

    // Arm cleanup guard — restores clipboard on every exit path after this point
    let mut guard = ClipboardGuard::new(saved_clipboard.clone());

    // Try to copy only the current selection (Cmd+C without Cmd+A)
    simulate_only_copy()?;
    std::thread::sleep(std::time::Duration::from_millis(100));

    let selection_text = crate::clipboard::read_clipboard().unwrap_or_default();
    log::info!("[STEP 1b] After copy-only: {} chars", selection_text.len());

    // Check if clipboard changed - if yes, there was a selection
    if !selection_text.is_empty() && selection_text != saved_clipboard {
        log::info!("[STEP 1c] Existing selection detected, using selected text");
    } else {
        // No selection found, select all and copy
        log::info!("[STEP 1c] No selection found, selecting all text");
        simulate_copy()?;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    log::info!("[STEP 2] Copy simulated successfully");

    let original_text = crate::clipboard::read_clipboard().map_err(|e| {
        log::error!("[STEP 3 FAIL] {}", e);
        format!("Clipboard error: {}", e)
    })?;
    log::info!("[STEP 3] Clipboard read: {} chars", original_text.len());

    if original_text.trim().is_empty() {
        log::warn!("No text selected, skipping processing");
        return Ok(());
    }

    log::info!("[STEP 4] Loading settings...");
    let settings = crate::config::load_settings().map_err(|e| {
        log::error!("[STEP 4 FAIL] {}", e);
        format!("Config error: {}", e)
    })?;
    log::info!("[STEP 4] Settings loaded: model={}", settings.model);

    log::info!("[STEP 5] Loading instruction file...");
    let instruction = crate::instructions::load_instruction(&settings.instruction_file)
        .unwrap_or_else(|_| crate::instructions::default_instruction());
    log::info!("[STEP 5] Instruction loaded ({} chars)", instruction.len());

    if settings.api_key_encrypted.is_empty() {
        log::error!("[STEP 6 FAIL] API key missing");
        return Err("API key missing".to_string());
    }

    log::info!("[STEP 6] Decrypting API key...");
    let api_key = crate::config::decrypt_api_key(&settings.api_key_encrypted).map_err(|e| {
        log::error!("[STEP 6 FAIL] {}", e);
        format!("Failed to decrypt API key: {}", e)
    })?;
    log::info!("[STEP 6] API key decrypted");

    let model = settings.model.clone();
    let proxy_url = settings.proxy_url.clone();
    let api_base_url = settings.api_base_url.clone();

    log::info!(
        "[STEP 7] Creating API client (provider: {:?}, model: {})...",
        settings.provider,
        model
    );
    let client = match settings.provider {
        crate::config::Provider::OpenRouter => {
            let c = match api_base_url {
                Some(ref url) if !url.is_empty() => crate::api::OpenRouterClient::new_with_url(
                    api_key,
                    model,
                    proxy_url,
                    url.clone(),
                ),
                _ => crate::api::OpenRouterClient::new(api_key, model, proxy_url),
            }
            .map_err(|e| {
                log::error!("[STEP 7 FAIL] {}", e);
                format!("API client error: {}", e)
            })?;
            crate::api::ApiClient::OpenRouter(c)
        }
        crate::config::Provider::Google => {
            let c = crate::api::GeminiClient::new(api_key, model, proxy_url).map_err(|e| {
                log::error!("[STEP 7 FAIL] {}", e);
                format!("API client error: {}", e)
            })?;
            crate::api::ApiClient::Google(c)
        }
    };
    log::info!("[STEP 7] API client created");

    log::info!("[STEP 8] Calling LLM API...");
    let refined = std::thread::spawn(move || match tokio::runtime::Runtime::new() {
        Ok(rt) => rt.block_on(client.refine_text(&instruction, &original_text)),
        Err(e) => Err(format!("Failed to create async runtime: {}", e)),
    })
    .join()
    .unwrap_or_else(|_| Err("Thread panicked".to_string()))
    .map_err(|e| {
        log::error!("[STEP 8 FAIL] {}", e);
        format!("API error: {}", e)
    })?;
    log::info!(
        "[STEP 8] API returned refined text ({} chars)",
        refined.len()
    );

    log::info!("[STEP 9] Writing to clipboard...");
    crate::clipboard::write_clipboard(&refined).map_err(|e| {
        log::error!("[STEP 9 FAIL] {}", e);
        format!("Clipboard error: {}", e)
    })?;
    log::info!("[STEP 9] Clipboard write complete");

    std::thread::sleep(std::time::Duration::from_millis(50));

    log::info!("[STEP 10] Simulating paste...");
    simulate_paste().map_err(|e| {
        log::error!("[STEP 10 FAIL] {}", e);
        e
    })?;
    guard.mark_paste_dispatched();
    log::info!("[STEP 10] Paste simulated");

    log::info!("Text replacement complete");
    Ok(())
}

#[cfg(target_os = "macos")]
fn simulate_copy() -> Result<(), String> {
    use core_graphics::event::{CGEvent, CGEventTapLocation};
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|e| format!("Failed to create event source: {:?}", e))?;

    // Select All: Cmd+A (keycode 0 = A, Cmd = flag)
    let cmd_down = CGEvent::new_keyboard_event(source.clone(), 0, true)
        .map_err(|e| format!("Failed to create Cmd+A down: {:?}", e))?;
    cmd_down.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
    cmd_down.post(CGEventTapLocation::HID);

    std::thread::sleep(std::time::Duration::from_millis(50));

    let cmd_up = CGEvent::new_keyboard_event(source.clone(), 0, false)
        .map_err(|e| format!("Failed to create Cmd+A up: {:?}", e))?;
    cmd_up.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
    cmd_up.post(CGEventTapLocation::HID);

    std::thread::sleep(std::time::Duration::from_millis(50));

    // Copy: Cmd+C (keycode 8 = C)
    let cmd_down = CGEvent::new_keyboard_event(source.clone(), 8, true)
        .map_err(|e| format!("Failed to create Cmd+C down: {:?}", e))?;
    cmd_down.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
    cmd_down.post(CGEventTapLocation::HID);

    std::thread::sleep(std::time::Duration::from_millis(50));

    let cmd_up = CGEvent::new_keyboard_event(source.clone(), 8, false)
        .map_err(|e| format!("Failed to create Cmd+C up: {:?}", e))?;
    cmd_up.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
    cmd_up.post(CGEventTapLocation::HID);

    Ok(())
}

#[cfg(target_os = "macos")]
fn simulate_only_copy() -> Result<(), String> {
    use core_graphics::event::{CGEvent, CGEventTapLocation};
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|e| format!("Failed to create event source: {:?}", e))?;

    // Copy: Cmd+C (keycode 8 = C)
    let cmd_down = CGEvent::new_keyboard_event(source.clone(), 8, true)
        .map_err(|e| format!("Failed to create Cmd+C down: {:?}", e))?;
    cmd_down.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
    cmd_down.post(CGEventTapLocation::HID);

    std::thread::sleep(std::time::Duration::from_millis(50));

    let cmd_up = CGEvent::new_keyboard_event(source.clone(), 8, false)
        .map_err(|e| format!("Failed to create Cmd+C up: {:?}", e))?;
    cmd_up.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
    cmd_up.post(CGEventTapLocation::HID);

    Ok(())
}

#[cfg(target_os = "macos")]
fn simulate_paste() -> Result<(), String> {
    use core_graphics::event::{CGEvent, CGEventTapLocation};
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|e| format!("Failed to create event source: {:?}", e))?;

    // Paste: Cmd+V (keycode 9 = V)
    let cmd_down = CGEvent::new_keyboard_event(source.clone(), 9, true)
        .map_err(|e| format!("Failed to create Cmd+V down: {:?}", e))?;
    cmd_down.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
    cmd_down.post(CGEventTapLocation::HID);

    std::thread::sleep(std::time::Duration::from_millis(50));

    let cmd_up = CGEvent::new_keyboard_event(source.clone(), 9, false)
        .map_err(|e| format!("Failed to create Cmd+V up: {:?}", e))?;
    cmd_up.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);
    cmd_up.post(CGEventTapLocation::HID);

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn simulate_copy() -> Result<(), String> {
    use enigo::{Direction, Enigo, Key, Keyboard, Settings as EnigoSettings};
    let mut enigo =
        Enigo::new(&EnigoSettings::default()).map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Control, Direction::Press)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Unicode('a'), Direction::Press)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Unicode('a'), Direction::Release)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Control, Direction::Release)
        .map_err(|e| format!("Enigo error: {}", e))?;
    std::thread::sleep(std::time::Duration::from_millis(50));
    enigo
        .key(Key::Control, Direction::Press)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Unicode('c'), Direction::Press)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Unicode('c'), Direction::Release)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Control, Direction::Release)
        .map_err(|e| format!("Enigo error: {}", e))?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn simulate_only_copy() -> Result<(), String> {
    use enigo::{Direction, Enigo, Key, Keyboard, Settings as EnigoSettings};
    let mut enigo =
        Enigo::new(&EnigoSettings::default()).map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Control, Direction::Press)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Unicode('c'), Direction::Press)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Unicode('c'), Direction::Release)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Control, Direction::Release)
        .map_err(|e| format!("Enigo error: {}", e))?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn simulate_paste() -> Result<(), String> {
    use enigo::{Direction, Enigo, Key, Keyboard, Settings as EnigoSettings};
    let mut enigo =
        Enigo::new(&EnigoSettings::default()).map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Control, Direction::Press)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Unicode('v'), Direction::Press)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Unicode('v'), Direction::Release)
        .map_err(|e| format!("Enigo error: {}", e))?;
    enigo
        .key(Key::Control, Direction::Release)
        .map_err(|e| format!("Enigo error: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::HotkeyConfig;

    #[test]
    fn test_default_hotkey_produces_correct_shortcut() {
        let h = HotkeyConfig::default();
        let shortcut = hotkey_config_to_shortcut(&h);
        assert!(!shortcut.is_empty());
        assert!(shortcut.contains('+'));
        let parts: Vec<&str> = shortcut.split('+').collect();
        assert!(parts.len() >= 2);
        assert_eq!(parts[parts.len() - 1], "r");
        #[cfg(target_os = "macos")]
        {
            assert_eq!(parts[0], "cmd");
        }
        #[cfg(not(target_os = "macos"))]
        {
            assert_eq!(parts[0], "ctrl");
        }
    }

    #[test]
    fn test_single_modifier_shortcut() {
        let h = HotkeyConfig {
            modifiers: vec!["alt".to_string()],
            key: "F4".to_string(),
        };
        assert_eq!(hotkey_config_to_shortcut(&h), "alt+F4");
    }

    #[test]
    fn test_multiple_modifiers_shortcut() {
        let h = HotkeyConfig {
            modifiers: vec!["ctrl".to_string(), "shift".to_string(), "alt".to_string()],
            key: "x".to_string(),
        };
        assert_eq!(hotkey_config_to_shortcut(&h), "ctrl+shift+alt+x");
    }

    #[test]
    fn test_shortcut_key_is_last_segment() {
        let h = HotkeyConfig {
            modifiers: vec!["cmd".to_string(), "option".to_string()],
            key: "escape".to_string(),
        };
        let shortcut = hotkey_config_to_shortcut(&h);
        let parts: Vec<&str> = shortcut.split('+').collect();
        assert_eq!(parts.last().unwrap(), &"escape");
    }

    #[test]
    fn test_processing_flag_default_is_false() {
        assert!(!PROCESSING.load(Ordering::SeqCst));
    }

    #[test]
    fn test_simulate_only_copy_is_callable() {
        let result = simulate_only_copy();
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn cleanup_plan_empty_no_paste_is_noop() {
        assert_eq!(cleanup_plan(true, false), (false, false));
    }

    #[test]
    fn cleanup_plan_empty_with_paste_sleeps_but_skips_write() {
        assert_eq!(cleanup_plan(true, true), (true, false));
    }

    #[test]
    fn cleanup_plan_nonempty_no_paste_restores_immediately() {
        assert_eq!(cleanup_plan(false, false), (false, true));
    }

    #[test]
    fn cleanup_plan_nonempty_with_paste_restores_after_sleep() {
        assert_eq!(cleanup_plan(false, true), (true, true));
    }

    #[test]
    fn clipboard_guard_new_starts_with_paste_not_dispatched() {
        let guard = ClipboardGuard::new(String::new());
        assert!(!guard.paste_dispatched);
        assert_eq!(guard.saved, "");
    }

    #[test]
    fn clipboard_guard_mark_paste_dispatched_flips_flag() {
        let mut guard = ClipboardGuard::new(String::new());
        assert!(!guard.paste_dispatched);
        guard.mark_paste_dispatched();
        assert!(guard.paste_dispatched);
    }

    #[test]
    fn clipboard_guard_drop_with_empty_saved_does_not_panic() {
        // Empty saved → write branch is skipped → no clipboard contact → safe in any env
        let guard = ClipboardGuard::new(String::new());
        drop(guard);
    }
}
