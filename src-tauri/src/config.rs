use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use log;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

const CONFIG_DIR: &str = "ghostwriter";
const CONFIG_FILE: &str = "config.json";
const ENCRYPTION_MAGIC: &[u8] = b"GHOSTWRITER_V1";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HotkeyConfig {
    pub modifiers: Vec<String>,
    pub key: String,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        {
            Self {
                modifiers: vec!["cmd".to_string()],
                key: "r".to_string(),
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            Self {
                modifiers: vec!["ctrl".to_string()],
                key: "r".to_string(),
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum Provider {
    #[default]
    #[serde(rename = "openrouter")]
    OpenRouter,
    #[serde(rename = "google")]
    Google,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(default)]
    pub api_key_encrypted: String,
    pub api_key: Option<String>,
    pub model: String,
    pub instruction_file: PathBuf,
    pub hotkey: HotkeyConfig,
    pub proxy_url: Option<String>,
    pub api_base_url: Option<String>,
    pub first_run: bool,
    #[serde(default)]
    pub provider: Provider,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key_encrypted: String::new(),
            api_key: None,
            model: "openai/gpt-4o-mini".to_string(),
            instruction_file: default_instruction_path(),
            hotkey: HotkeyConfig::default(),
            proxy_url: None,
            api_base_url: None,
            first_run: true,
            provider: Provider::default(),
        }
    }
}

pub fn config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join(CONFIG_DIR)
}

fn config_path() -> PathBuf {
    config_dir().join(CONFIG_FILE)
}

/// Returns the legacy config directory path for backward compatibility
/// On macOS: ~/Library/Application Support/ghostwriter
/// On Linux/Windows: Same as config_dir() for consistency
fn legacy_config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Library")
            .join("Application Support")
            .join(CONFIG_DIR)
    }
    #[cfg(not(target_os = "macos"))]
    {
        // On non-macOS platforms, legacy and current paths are the same
        config_dir()
    }
}

/// Returns the legacy config file path for backward compatibility
fn legacy_config_path() -> PathBuf {
    legacy_config_dir().join(CONFIG_FILE)
}

/// Migrates settings from legacy location to new XDG location if needed
/// Returns Ok(()) if migration succeeded or wasn't needed
/// Returns Err if migration failed
pub fn migrate_legacy_config_if_needed() -> Result<(), ConfigError> {
    let legacy_path = legacy_config_path();
    let new_path = config_path();

    // Only migrate if legacy file exists and new file doesn't exist
    if legacy_path.exists() && !new_path.exists() {
        // Ensure new config directory exists
        let new_dir = config_dir();
        fs::create_dir_all(&new_dir)?;

        // Copy the file
        fs::copy(&legacy_path, &new_path)?;

        log::info!(
            "Migrated config from {} to {}",
            legacy_path.display(),
            new_path.display()
        );

        // Optionally remove legacy file after successful migration
        // Commenting out for safety - user can manually remove if desired
        // fs::remove_file(&legacy_path)?;
    }

    Ok(())
}

fn default_instruction_path() -> PathBuf {
    config_dir().join("instructions").join("default.md")
}

pub fn load_settings() -> Result<Settings, ConfigError> {
    load_settings_at(&config_path())
}

pub(crate) fn load_settings_at(path: &Path) -> Result<Settings, ConfigError> {
    if !path.exists() {
        let default_settings = Settings::default();
        save_settings_at(path, &default_settings)?;
        return Ok(default_settings);
    }

    let content = fs::read_to_string(path)?;
    let mut settings: Settings =
        serde_json::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
    validate_settings_urls(&mut settings);
    Ok(settings)
}

pub fn save_settings(settings: &Settings) -> Result<(), ConfigError> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;
    save_settings_at(&config_path(), settings)
}

pub(crate) fn save_settings_at(path: &Path, settings: &Settings) -> Result<(), ConfigError> {
    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| ConfigError::SerializeError(e.to_string()))?;
    fs::write(path, content)?;
    Ok(())
}

fn validate_settings_urls(settings: &mut Settings) {
    if let Some(ref url_str) = settings.api_base_url.clone() {
        match Url::parse(url_str) {
            Ok(parsed) if matches!(parsed.scheme(), "http" | "https") => {}
            Ok(parsed) => {
                log::warn!(
                    "api_base_url has unsupported scheme '{}', must be http or https — ignoring",
                    parsed.scheme()
                );
                settings.api_base_url = None;
            }
            Err(e) => {
                log::warn!(
                    "api_base_url '{}' is not a valid URL: {} — ignoring",
                    url_str,
                    e
                );
                settings.api_base_url = None;
            }
        }
    }

    if let Some(ref url_str) = settings.proxy_url.clone() {
        match Url::parse(url_str) {
            Ok(parsed) if matches!(parsed.scheme(), "http" | "https" | "socks5") => {}
            Ok(parsed) => {
                log::warn!(
                    "proxy_url has unsupported scheme '{}', must be http, https, or socks5 — ignoring",
                    parsed.scheme()
                );
                settings.proxy_url = None;
            }
            Err(e) => {
                log::warn!(
                    "proxy_url '{}' is not a valid URL: {} — ignoring",
                    url_str,
                    e
                );
                settings.proxy_url = None;
            }
        }
    }
}

pub fn encrypt_api_key(api_key: &str) -> Result<String, ConfigError> {
    let machine_key = get_machine_key();
    let encrypted: Vec<u8> = api_key
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ machine_key[i % machine_key.len()])
        .collect();

    let mut result = ENCRYPTION_MAGIC.to_vec();
    result.extend(encrypted);
    Ok(BASE64.encode(&result))
}

pub fn decrypt_api_key(encrypted: &str) -> Result<String, ConfigError> {
    let data = BASE64
        .decode(encrypted)
        .map_err(|e| ConfigError::DecryptionError(e.to_string()))?;

    if data.starts_with(ENCRYPTION_MAGIC) {
        let encrypted_bytes = &data[ENCRYPTION_MAGIC.len()..];
        let machine_key = get_machine_key();
        let decrypted: Vec<u8> = encrypted_bytes
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ machine_key[i % machine_key.len()])
            .collect();
        String::from_utf8(decrypted).map_err(|e| ConfigError::DecryptionError(e.to_string()))
    } else {
        Err(ConfigError::InvalidKeyFormat)
    }
}

fn get_machine_key() -> Vec<u8> {
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "ghostwriter".to_string());
    hostname.as_bytes().to_vec()
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Serialize error: {0}")]
    SerializeError(String),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    #[error("Invalid encrypted key format")]
    InvalidKeyFormat,
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Settings(provider: {:?}, model: {}, instruction: {:?}, api_base_url: {}, has_raw_key: {})",
            self.provider,
            self.model,
            self.instruction_file,
            self.api_base_url
                .as_deref()
                .unwrap_or("default"),
            self.api_key.is_some()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn temp_config_path() -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.json");
        (dir, path)
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = "sk-or-v1-test123456789";
        let encrypted = encrypt_api_key(key).unwrap();
        let decrypted = decrypt_api_key(&encrypted).unwrap();
        assert_eq!(decrypted, key);
    }

    #[test]
    fn test_encrypt_decrypt_empty_key() {
        let key = "";
        let encrypted = encrypt_api_key(key).unwrap();
        let decrypted = decrypt_api_key(&encrypted).unwrap();
        assert_eq!(decrypted, key);
    }

    #[test]
    fn test_encrypt_decrypt_special_chars() {
        let key = "sk-or-v1-!@#$%^&*()_+-=[]{}|;':\",./<>?`~你好";
        let encrypted = encrypt_api_key(key).unwrap();
        let decrypted = decrypt_api_key(&encrypted).unwrap();
        assert_eq!(decrypted, key);
    }

    #[test]
    fn test_encrypt_output_has_magic_bytes() {
        let key = "test-key";
        let encrypted = encrypt_api_key(key).unwrap();
        let decoded = BASE64.decode(&encrypted).unwrap();
        assert!(decoded.starts_with(ENCRYPTION_MAGIC));
    }

    #[test]
    fn test_decrypt_invalid_base64() {
        let result = decrypt_api_key("not-valid-base64!!!");
        assert!(result.is_err());
        match result {
            Err(ConfigError::DecryptionError(_)) => {}
            _ => panic!("Expected DecryptionError"),
        }
    }

    #[test]
    fn test_decrypt_no_magic_bytes() {
        let encoded = BASE64.encode(b"no-magic-here");
        let result = decrypt_api_key(&encoded);
        assert!(result.is_err());
        match result {
            Err(ConfigError::InvalidKeyFormat) => {}
            _ => panic!("Expected InvalidKeyFormat"),
        }
    }

    #[test]
    fn test_decrypt_empty_string() {
        let result = decrypt_api_key("");
        assert!(result.is_err());
    }

    #[test]
    fn test_settings_default_values() {
        let s = Settings::default();
        assert_eq!(s.model, "openai/gpt-4o-mini");
        assert!(s.first_run);
        assert!(s.api_key_encrypted.is_empty());
        assert!(s.api_key.is_none());
        assert_eq!(s.hotkey.key, "r");
        assert!(s.proxy_url.is_none());
        assert!(s.api_base_url.is_none());
    }

    #[test]
    fn test_settings_default_hotkey_platform() {
        let h = HotkeyConfig::default();
        assert_eq!(h.key, "r");
        #[cfg(target_os = "macos")]
        assert_eq!(h.modifiers, vec!["cmd".to_string()]);
        #[cfg(not(target_os = "macos"))]
        assert_eq!(h.modifiers, vec!["ctrl".to_string()]);
    }

    #[test]
    fn test_settings_serde_roundtrip() {
        let s = Settings {
            api_key_encrypted: "encrypted_value".to_string(),
            api_key: Some("raw_key".to_string()),
            model: "anthropic/claude-3".to_string(),
            instruction_file: PathBuf::from("/tmp/test.md"),
            hotkey: HotkeyConfig {
                modifiers: vec!["alt".to_string()],
                key: "t".to_string(),
            },
            proxy_url: Some("http://proxy:8080".to_string()),
            api_base_url: Some("https://opencode.ai/zen/v1/chat/completions".to_string()),
            first_run: false,
            provider: Provider::OpenRouter,
        };

        let json = serde_json::to_string_pretty(&s).unwrap();
        let deserialized: Settings = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.model, s.model);
        assert_eq!(deserialized.api_key_encrypted, s.api_key_encrypted);
        assert_eq!(deserialized.api_key, s.api_key);
        assert_eq!(deserialized.instruction_file, s.instruction_file);
        assert_eq!(deserialized.hotkey.key, s.hotkey.key);
        assert_eq!(deserialized.hotkey.modifiers, s.hotkey.modifiers);
        assert_eq!(deserialized.proxy_url, s.proxy_url);
        assert_eq!(deserialized.api_base_url, s.api_base_url);
        assert_eq!(deserialized.first_run, s.first_run);
        assert_eq!(deserialized.provider, Provider::OpenRouter);
    }

    #[test]
    fn test_load_settings_at_missing_file() {
        let (_dir, path) = temp_config_path();
        assert!(!path.exists());

        let result = load_settings_at(&path).unwrap();
        assert!(result.first_run);
        assert!(path.exists());
    }

    #[test]
    fn test_save_and_load_settings_at() {
        let (_dir, path) = temp_config_path();

        let settings = Settings {
            api_key_encrypted: "test_enc".to_string(),
            api_key: None,
            model: "test-model".to_string(),
            instruction_file: PathBuf::from("/tmp/test.md"),
            hotkey: HotkeyConfig::default(),
            proxy_url: None,
            api_base_url: None,
            first_run: false,
            provider: Provider::OpenRouter,
        };

        save_settings_at(&path, &settings).unwrap();
        assert!(path.exists());

        let loaded = load_settings_at(&path).unwrap();
        assert_eq!(loaded.model, "test-model");
        assert_eq!(loaded.api_key_encrypted, "test_enc");
        assert!(!loaded.first_run);
    }

    #[test]
    fn test_load_settings_at_invalid_json() {
        let (_dir, path) = temp_config_path();
        fs::write(&path, "this is not json").unwrap();

        let result = load_settings_at(&path);
        assert!(result.is_err());
        match result {
            Err(ConfigError::ParseError(_)) => {}
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_config_dir_format() {
        let dir = config_dir();
        let dir_str = dir.to_string_lossy();
        assert!(dir_str.contains(".config/ghostwriter"));
        assert!(dir.is_absolute());
    }

    #[test]
    fn test_legacy_config_dir_macos() {
        #[cfg(target_os = "macos")]
        {
            let dir = legacy_config_dir();
            let dir_str = dir.to_string_lossy();
            assert!(dir_str.contains("Library/Application Support/ghostwriter"));
            assert!(dir.is_absolute());
        }
        #[cfg(not(target_os = "macos"))]
        {
            // On non-macOS, legacy and current paths should be the same
            let legacy_dir = legacy_config_dir();
            let current_dir = config_dir();
            assert_eq!(legacy_dir, current_dir);
        }
    }

    #[test]
    fn test_display_does_not_leak_key() {
        let s = Settings {
            api_key_encrypted: "secret-key-value".to_string(),
            api_key: None,
            model: "gpt-4".to_string(),
            instruction_file: PathBuf::from("/tmp/test.md"),
            hotkey: HotkeyConfig::default(),
            proxy_url: None,
            api_base_url: None,
            first_run: false,
            provider: Provider::OpenRouter,
        };

        let display = s.to_string();
        assert!(display.contains("gpt-4"));
        assert!(display.contains("default"));
        assert!(display.contains("has_raw_key: false"));
        assert!(!display.contains("secret-key-value"));
    }

    #[test]
    fn test_error_display() {
        let err = ConfigError::DecryptionError("bad decrypt".to_string());
        assert_eq!(err.to_string(), "Decryption error: bad decrypt");

        let err = ConfigError::InvalidKeyFormat;
        assert_eq!(err.to_string(), "Invalid encrypted key format");

        let err = ConfigError::ParseError("bad json".to_string());
        assert_eq!(err.to_string(), "Parse error: bad json");
    }

    fn settings_with_urls(api_base_url: Option<&str>, proxy_url: Option<&str>) -> Settings {
        Settings {
            api_base_url: api_base_url.map(str::to_string),
            proxy_url: proxy_url.map(str::to_string),
            ..Settings::default()
        }
    }

    #[test]
    fn test_validate_urls_valid_http_base_url() {
        let mut s = settings_with_urls(Some("http://custom.api/v1/chat"), None);
        validate_settings_urls(&mut s);
        assert_eq!(s.api_base_url.as_deref(), Some("http://custom.api/v1/chat"));
    }

    #[test]
    fn test_validate_urls_valid_https_base_url() {
        let mut s = settings_with_urls(Some("https://openrouter.ai/api/v1/chat/completions"), None);
        validate_settings_urls(&mut s);
        assert!(s.api_base_url.is_some());
    }

    #[test]
    fn test_validate_urls_invalid_scheme_base_url() {
        let mut s = settings_with_urls(Some("ftp://files.example.com/api"), None);
        validate_settings_urls(&mut s);
        assert!(s.api_base_url.is_none());
    }

    #[test]
    fn test_validate_urls_socks5_rejected_for_base_url() {
        let mut s = settings_with_urls(Some("socks5://proxy.example.com:1080"), None);
        validate_settings_urls(&mut s);
        assert!(s.api_base_url.is_none());
    }

    #[test]
    fn test_validate_urls_malformed_base_url() {
        let mut s = settings_with_urls(Some("not a url at all"), None);
        validate_settings_urls(&mut s);
        assert!(s.api_base_url.is_none());
    }

    #[test]
    fn test_validate_urls_empty_string_base_url() {
        let mut s = settings_with_urls(Some(""), None);
        validate_settings_urls(&mut s);
        assert!(s.api_base_url.is_none());
    }

    #[test]
    fn test_validate_urls_valid_http_proxy() {
        let mut s = settings_with_urls(None, Some("http://proxy.example.com:8080"));
        validate_settings_urls(&mut s);
        assert!(s.proxy_url.is_some());
    }

    #[test]
    fn test_validate_urls_valid_https_proxy() {
        let mut s = settings_with_urls(None, Some("https://proxy.example.com:8080"));
        validate_settings_urls(&mut s);
        assert!(s.proxy_url.is_some());
    }

    #[test]
    fn test_validate_urls_valid_socks5_proxy() {
        let mut s = settings_with_urls(None, Some("socks5://proxy.example.com:1080"));
        validate_settings_urls(&mut s);
        assert!(s.proxy_url.is_some());
    }

    #[test]
    fn test_validate_urls_invalid_scheme_proxy() {
        let mut s = settings_with_urls(None, Some("ftp://proxy.example.com"));
        validate_settings_urls(&mut s);
        assert!(s.proxy_url.is_none());
    }

    #[test]
    fn test_validate_urls_malformed_proxy() {
        let mut s = settings_with_urls(None, Some(":::bad:::"));
        validate_settings_urls(&mut s);
        assert!(s.proxy_url.is_none());
    }

    #[test]
    fn test_validate_urls_none_fields_unchanged() {
        let mut s = settings_with_urls(None, None);
        validate_settings_urls(&mut s);
        assert!(s.api_base_url.is_none());
        assert!(s.proxy_url.is_none());
    }

    #[test]
    fn test_load_settings_at_clears_invalid_base_url() {
        let (_dir, path) = temp_config_path();
        let bad = r#"{
            "api_key_encrypted": "",
            "model": "gpt-4",
            "instruction_file": "/tmp/test.md",
            "hotkey": {"modifiers": ["cmd"], "key": "r"},
            "api_base_url": "ftp://bad.scheme/api",
            "first_run": false
        }"#;
        fs::write(&path, bad).unwrap();
        let loaded = load_settings_at(&path).unwrap();
        assert!(loaded.api_base_url.is_none());
    }

    #[test]
    fn test_load_settings_at_clears_invalid_proxy_url() {
        let (_dir, path) = temp_config_path();
        let bad = r#"{
            "api_key_encrypted": "",
            "model": "gpt-4",
            "instruction_file": "/tmp/test.md",
            "hotkey": {"modifiers": ["cmd"], "key": "r"},
            "proxy_url": "not-a-url",
            "first_run": false
        }"#;
        fs::write(&path, bad).unwrap();
        let loaded = load_settings_at(&path).unwrap();
        assert!(loaded.proxy_url.is_none());
    }

    #[test]
    fn test_provider_default_is_openrouter() {
        assert_eq!(Provider::default(), Provider::OpenRouter);
    }

    #[test]
    fn test_provider_serde_roundtrip() {
        let json = r#""openrouter""#;
        let p: Provider = serde_json::from_str(json).unwrap();
        assert_eq!(p, Provider::OpenRouter);

        let json = r#""google""#;
        let p: Provider = serde_json::from_str(json).unwrap();
        assert_eq!(p, Provider::Google);
    }

    #[test]
    fn test_old_config_without_provider_defaults_to_openrouter() {
        let (_dir, path) = temp_config_path();
        let old_json = r#"{
            "api_key_encrypted": "",
            "model": "gpt-4",
            "instruction_file": "/tmp/test.md",
            "hotkey": {"modifiers": ["cmd"], "key": "r"},
            "first_run": false
        }"#;
        fs::write(&path, old_json).unwrap();
        let loaded = load_settings_at(&path).unwrap();
        assert_eq!(loaded.provider, Provider::OpenRouter);
    }

    #[test]
    fn test_provider_google_config_loads_correctly() {
        let (_dir, path) = temp_config_path();
        let json = r#"{
            "api_key_encrypted": "enc_key",
            "model": "gemini-2.0-flash",
            "instruction_file": "/tmp/test.md",
            "hotkey": {"modifiers": ["cmd"], "key": "r"},
            "proxy_url": null,
            "api_base_url": null,
            "first_run": false,
            "provider": "google"
        }"#;
        fs::write(&path, json).unwrap();
        let loaded = load_settings_at(&path).unwrap();
        assert_eq!(loaded.provider, Provider::Google);
        assert_eq!(loaded.model, "gemini-2.0-flash");
    }
}
