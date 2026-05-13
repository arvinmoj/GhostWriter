use arboard::Clipboard;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static CLIPBOARD: Lazy<Mutex<Clipboard>> = Lazy::new(|| {
    Mutex::new(Clipboard::new().expect("Failed to access clipboard"))
});

pub fn read_clipboard() -> Result<String, ClipboardError> {
    let mut clipboard = CLIPBOARD.lock()
        .map_err(|_| ClipboardError::LockError)?;
    let text = clipboard.get_text()
        .map_err(|e| ClipboardError::ReadError(e.to_string()))?;
    Ok(text)
}

pub fn write_clipboard(text: &str) -> Result<(), ClipboardError> {
    let mut clipboard = CLIPBOARD.lock()
        .map_err(|_| ClipboardError::LockError)?;
    clipboard.set_text(text)
        .map_err(|e| ClipboardError::WriteError(e.to_string()))?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ClipboardError {
    #[error("Failed to acquire clipboard lock")]
    LockError,
    #[error("Failed to read clipboard: {0}")]
    ReadError(String),
    #[error("Failed to write clipboard: {0}")]
    WriteError(String),
}
