use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub api_key_encrypted: String,
    pub model: String,
    pub instruction_file: PathBuf,
    pub hotkey: HotkeyConfig,
    pub proxy_url: Option<String>,
    pub first_run: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key_encrypted: String::new(),
            model: "openai/gpt-4o-mini".to_string(),
            instruction_file: default_instruction_path(),
            hotkey: HotkeyConfig::default(),
            proxy_url: None,
            first_run: true,
        }
    }
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(CONFIG_DIR)
}

fn config_path() -> PathBuf {
    config_dir().join(CONFIG_FILE)
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
    let settings: Settings =
        serde_json::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
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
            "Settings(model: {}, instruction: {:?})",
            self.model, self.instruction_file
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
        assert_eq!(s.hotkey.key, "r");
        assert!(s.proxy_url.is_none());
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
            model: "anthropic/claude-3".to_string(),
            instruction_file: PathBuf::from("/tmp/test.md"),
            hotkey: HotkeyConfig {
                modifiers: vec!["alt".to_string()],
                key: "t".to_string(),
            },
            proxy_url: Some("http://proxy:8080".to_string()),
            first_run: false,
        };

        let json = serde_json::to_string_pretty(&s).unwrap();
        let deserialized: Settings = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.model, s.model);
        assert_eq!(deserialized.api_key_encrypted, s.api_key_encrypted);
        assert_eq!(deserialized.instruction_file, s.instruction_file);
        assert_eq!(deserialized.hotkey.key, s.hotkey.key);
        assert_eq!(deserialized.hotkey.modifiers, s.hotkey.modifiers);
        assert_eq!(deserialized.proxy_url, s.proxy_url);
        assert_eq!(deserialized.first_run, s.first_run);
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
            model: "test-model".to_string(),
            instruction_file: PathBuf::from("/tmp/test.md"),
            hotkey: HotkeyConfig::default(),
            proxy_url: None,
            first_run: false,
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
        assert!(dir_str.contains("ghostwriter"));
        assert!(dir.is_absolute());
    }

    #[test]
    fn test_display_does_not_leak_key() {
        let s = Settings {
            api_key_encrypted: "secret-key-value".to_string(),
            model: "gpt-4".to_string(),
            instruction_file: PathBuf::from("/tmp/test.md"),
            hotkey: HotkeyConfig::default(),
            proxy_url: None,
            first_run: false,
        };

        let display = s.to_string();
        assert!(display.contains("gpt-4"));
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
}
