use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

const LOG_FILE: &str = "ghostwriter.log";
const MAX_BODY_LEN: usize = 240;

pub fn error(app: &AppHandle, title: &str, body: &str) {
    let body = truncate(body);
    log::error!("{}: {}", title, body);

    let shown = app
        .notification()
        .builder()
        .title(title)
        .body(&body)
        .show()
        .is_ok();

    if !shown {
        fallback_log(title, &body);
    }
}

pub fn classify(err: &str) -> (&'static str, String) {
    let lower = err.to_lowercase();
    if lower.contains("api key missing") || lower.contains("decrypt") {
        (
            "API key missing",
            "Set your OpenRouter API key in ~/.config/ghostwriter/config.json.".to_string(),
        )
    } else if lower.contains("api error")
        || lower.contains("network error")
        || lower.contains("parse error")
        || lower.contains("no response from api")
    {
        ("AI request failed", err.to_string())
    } else if lower.contains("api client") || lower.contains("http client") {
        ("API client error", err.to_string())
    } else if lower.contains("clipboard") {
        ("Clipboard error", err.to_string())
    } else if lower.contains("config") {
        ("Configuration error", err.to_string())
    } else {
        ("GhostWriter error", err.to_string())
    }
}

fn truncate(s: &str) -> String {
    if s.len() <= MAX_BODY_LEN {
        s.to_string()
    } else {
        let mut end = MAX_BODY_LEN;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}…", &s[..end])
    }
}

fn log_path() -> PathBuf {
    crate::config::config_dir().join(LOG_FILE)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn format_log_line(ts: u64, title: &str, body: &str) -> String {
    format!("{} [ERROR] {}: {}\n", ts, title, body)
}

fn fallback_log(title: &str, body: &str) {
    fallback_log_to(&log_path(), now_secs(), title, body);
}

fn fallback_log_to(path: &Path, ts: u64, title: &str, body: &str) {
    let line = format_log_line(ts, title, body);
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_missing_api_key_literal() {
        let (title, body) = classify("API key missing");
        assert_eq!(title, "API key missing");
        assert!(body.contains("config.json"));
    }

    #[test]
    fn classify_decrypt_failure_maps_to_missing_key() {
        let (title, _) = classify("Failed to decrypt API key: invalid data");
        assert_eq!(title, "API key missing");
    }

    #[test]
    fn classify_api_error() {
        let (title, body) = classify("API error: 401 - Unauthorized");
        assert_eq!(title, "AI request failed");
        assert!(body.contains("401"));
    }

    #[test]
    fn classify_network_error() {
        let (title, _) = classify("Network error: connection refused");
        assert_eq!(title, "AI request failed");
    }

    #[test]
    fn classify_parse_error() {
        let (title, _) = classify("Parse error: bad json");
        assert_eq!(title, "AI request failed");
    }

    #[test]
    fn classify_no_response_from_api() {
        let (title, _) = classify("No response from API");
        assert_eq!(title, "AI request failed");
    }

    #[test]
    fn classify_api_client_error() {
        let (title, body) = classify("API client error: HTTP client build failed");
        assert_eq!(title, "API client error");
        assert!(body.contains("HTTP client"));
    }

    #[test]
    fn classify_http_client_error() {
        let (title, _) = classify("Failed to create HTTP client: bad TLS");
        assert_eq!(title, "API client error");
    }

    #[test]
    fn classify_clipboard_error() {
        let (title, _) = classify("Clipboard error: access denied");
        assert_eq!(title, "Clipboard error");
    }

    #[test]
    fn classify_config_error() {
        let (title, _) = classify("Config error: malformed json");
        assert_eq!(title, "Configuration error");
    }

    #[test]
    fn classify_fallback_for_unknown() {
        let (title, body) = classify("Something else went wrong");
        assert_eq!(title, "GhostWriter error");
        assert_eq!(body, "Something else went wrong");
    }

    #[test]
    fn classify_is_case_insensitive() {
        assert_eq!(classify("CLIPBOARD ERROR").0, "Clipboard error");
        assert_eq!(classify("Api Error: 500").0, "AI request failed");
    }

    #[test]
    fn classify_precedence_decrypt_beats_config() {
        let (title, _) = classify("Config error: Failed to decrypt API key");
        assert_eq!(title, "API key missing");
    }

    #[test]
    fn truncate_empty_string() {
        assert_eq!(truncate(""), "");
    }

    #[test]
    fn truncate_short_passthrough() {
        assert_eq!(truncate("hello"), "hello");
    }

    #[test]
    fn truncate_exactly_at_limit() {
        let s = "a".repeat(MAX_BODY_LEN);
        assert_eq!(truncate(&s), s);
    }

    #[test]
    fn truncate_one_past_limit_gets_ellipsis() {
        let s = "a".repeat(MAX_BODY_LEN + 1);
        let out = truncate(&s);
        assert!(out.ends_with('…'));
        assert_eq!(out.chars().filter(|c| *c == 'a').count(), MAX_BODY_LEN);
    }

    #[test]
    fn truncate_long_string() {
        let long = "a".repeat(MAX_BODY_LEN + 50);
        let out = truncate(&long);
        assert!(out.ends_with('…'));
        assert!(out.len() <= MAX_BODY_LEN + 4);
    }

    #[test]
    fn truncate_respects_multibyte_char_boundary() {
        let s = format!("{}é{}", "a".repeat(MAX_BODY_LEN - 1), "b".repeat(50));
        let out = truncate(&s);
        assert!(out.ends_with('…'));
        assert!(std::str::from_utf8(out.as_bytes()).is_ok());
    }

    #[test]
    fn format_log_line_shape() {
        let line = format_log_line(42, "Title", "Body");
        assert_eq!(line, "42 [ERROR] Title: Body\n");
    }

    #[test]
    fn fallback_log_to_creates_file_and_writes_line() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join(LOG_FILE);
        fallback_log_to(&path, 100, "Boom", "Things broke");
        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "100 [ERROR] Boom: Things broke\n");
    }

    #[test]
    fn fallback_log_to_appends_rather_than_overwrites() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join(LOG_FILE);
        fallback_log_to(&path, 1, "A", "first");
        fallback_log_to(&path, 2, "B", "second");
        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "1 [ERROR] A: first\n2 [ERROR] B: second\n");
    }

    #[test]
    fn fallback_log_to_silent_on_unwritable_path() {
        let bogus = Path::new("/this/path/does/not/exist/ghostwriter.log");
        fallback_log_to(bogus, 0, "ignored", "ignored");
    }

    #[test]
    fn now_secs_is_nonzero() {
        assert!(now_secs() > 0);
    }
}
