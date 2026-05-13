use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
    let path = config_path();

    if !path.exists() {
        let default_settings = Settings::default();
        save_settings(&default_settings)?;
        return Ok(default_settings);
    }

    let content = fs::read_to_string(&path)?;
    let settings: Settings = serde_json::from_str(&content)
        .map_err(|e| ConfigError::ParseError(e.to_string()))?;
    Ok(settings)
}

pub fn save_settings(settings: &Settings) -> Result<(), ConfigError> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;

    let path = config_path();
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
    let data = BASE64.decode(encrypted)
        .map_err(|e| ConfigError::DecryptionError(e.to_string()))?;

    if data.starts_with(ENCRYPTION_MAGIC) {
        let encrypted_bytes = &data[ENCRYPTION_MAGIC.len()..];
        let machine_key = get_machine_key();
        let decrypted: Vec<u8> = encrypted_bytes
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ machine_key[i % machine_key.len()])
            .collect();
        String::from_utf8(decrypted)
            .map_err(|e| ConfigError::DecryptionError(e.to_string()))
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
        write!(f, "Settings(model: {}, instruction: {:?})", self.model, self.instruction_file)
    }
}
