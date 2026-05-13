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
    pub first_run: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key_encrypted: String::new(),
            model: "openai/gpt-4o-mini".to_string(),
            instruction_file: default_instruction_path(),
            hotkey: HotkeyConfig::default(),
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
