use ghostwriter_lib::clipboard::{read_clipboard, write_clipboard};
use serial_test::serial;

fn clipboard_available() -> bool {
    std::env::var("DISPLAY").is_ok() || !cfg!(target_os = "linux")
}

#[test]
#[serial]
fn test_write_then_read() {
    if !clipboard_available() {
        eprintln!("Skipping clipboard test (no display server)");
        return;
    }
    let text = "GhostWriter test content";
    write_clipboard(text).unwrap();
    let result = read_clipboard().unwrap();
    assert_eq!(result, text);
}

#[test]
#[serial]
fn test_unicode_content() {
    if !clipboard_available() {
        eprintln!("Skipping clipboard test (no display server)");
        return;
    }
    let text = "Hello, 世界! 🌍✨ Привет";
    write_clipboard(text).unwrap();
    let result = read_clipboard().unwrap();
    assert_eq!(result, text);
}

#[test]
#[serial]
fn test_write_multiple_times() {
    if !clipboard_available() {
        eprintln!("Skipping clipboard test (no display server)");
        return;
    }
    write_clipboard("first").unwrap();
    assert_eq!(read_clipboard().unwrap(), "first");

    write_clipboard("second").unwrap();
    assert_eq!(read_clipboard().unwrap(), "second");

    write_clipboard("third").unwrap();
    assert_eq!(read_clipboard().unwrap(), "third");
}

#[test]
#[serial]
fn test_empty_string() {
    if !clipboard_available() {
        eprintln!("Skipping clipboard test (no display server)");
        return;
    }
    write_clipboard("").unwrap();
    let result = read_clipboard().unwrap();
    assert_eq!(result, "");
}
