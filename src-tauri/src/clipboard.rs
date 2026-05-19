use arboard::Clipboard;
use std::sync::Mutex;

static CLIPBOARD: Mutex<Option<Clipboard>> = Mutex::new(None);

fn get_clipboard() -> Result<std::sync::MutexGuard<'static, Option<Clipboard>>, ClipboardError> {
    let mut guard = CLIPBOARD.lock().map_err(|_| ClipboardError::LockError)?;
    if guard.is_none() {
        *guard = Some(Clipboard::new().map_err(|e| ClipboardError::InitError(e.to_string()))?);
    }
    Ok(guard)
}

pub fn read_clipboard() -> Result<String, ClipboardError> {
    let mut guard = get_clipboard()?;
    let clipboard = guard
        .as_mut()
        .ok_or_else(|| ClipboardError::InitError("Clipboard not initialized".into()))?;
    let text = clipboard
        .get_text()
        .map_err(|e| ClipboardError::ReadError(e.to_string()))?;
    Ok(text)
}

pub fn write_clipboard(text: &str) -> Result<(), ClipboardError> {
    let mut guard = get_clipboard()?;
    let clipboard = guard
        .as_mut()
        .ok_or_else(|| ClipboardError::InitError("Clipboard not initialized".into()))?;
    clipboard
        .set_text(text)
        .map_err(|e| ClipboardError::WriteError(e.to_string()))?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ClipboardError {
    #[error("Failed to acquire clipboard lock")]
    LockError,
    #[error("Failed to initialize clipboard: {0}")]
    InitError(String),
    #[error("Failed to read clipboard: {0}")]
    ReadError(String),
    #[error("Failed to write clipboard: {0}")]
    WriteError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ClipboardError::LockError;
        assert_eq!(err.to_string(), "Failed to acquire clipboard lock");

        let err = ClipboardError::ReadError("no data".to_string());
        assert_eq!(err.to_string(), "Failed to read clipboard: no data");

        let err = ClipboardError::WriteError("permission denied".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to write clipboard: permission denied"
        );

        let err = ClipboardError::InitError("no display".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to initialize clipboard: no display"
        );
    }
}
