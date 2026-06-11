use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};
use tauri_plugin_opener::OpenerExt;

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_compiles() {
        // Compile verification — tray requires a running Tauri app
        assert!(true);
    }
}

pub fn create_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let quit = MenuItem::with_id(app, "quit", "Quit GhostWriter", true, None::<&str>)?;
    let about = MenuItem::with_id(app, "about", "About GhostWriter", true, None::<&str>)?;
    let separator = MenuItem::with_id(app, "sep", "─────────────", false, None::<&str>)?;

    let menu = Menu::with_items(app, &[&about, &separator, &quit])?;

    let icon_bytes = include_bytes!("../icons/icon.png");
    let icon = Image::from_bytes(icon_bytes)?;
    log::info!(
        "Loaded tray icon successfully, size: {} bytes",
        icon_bytes.len()
    );

    let tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .tooltip("GhostWriter - Press Cmd+Shift+R to transform text")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                log::info!("Quit requested from tray");
                app.exit(0);
            }
            "about" => {
                log::info!("About clicked");
                let app = app.clone();
                std::thread::spawn(move || {
                    use tauri_plugin_dialog::DialogExt;
                    let body = crate::about::dialog_body();
                    let result = app
                        .dialog()
                        .message(&body)
                        .title(crate::about::DIALOG_TITLE)
                        .kind(tauri_plugin_dialog::MessageDialogKind::Info)
                        .buttons(tauri_plugin_dialog::MessageDialogButtons::OkCancelCustom(
                            crate::about::OPEN_REPO_BUTTON.to_string(),
                            crate::about::OK_BUTTON.to_string(),
                        ))
                        .blocking_show();
                    if result {
                        if let Err(e) = app.opener().open_url(crate::about::REPO_URL, None::<&str>)
                        {
                            log::warn!("Failed to open repo URL: {}", e);
                        }
                    }
                });
            }
            _ => {}
        })
        .on_tray_icon_event(|_tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                log::info!("Tray icon left-clicked");
            }
        })
        .build(app)?;

    app.manage(tray);

    log::info!("System tray created");
    Ok(())
}
