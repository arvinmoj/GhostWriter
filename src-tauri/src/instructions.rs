use std::fs;
use std::path::Path;

pub fn load_instruction(path: &Path) -> Result<String, InstructionError> {
    if !path.exists() {
        return Err(InstructionError::FileNotFound(path.display().to_string()));
    }

    let content = fs::read_to_string(path)?;

    if content.trim().is_empty() {
        return Err(InstructionError::EmptyFile(path.display().to_string()));
    }

    Ok(content)
}

pub fn default_instruction() -> String {
    r#"You are a helpful AI assistant. Improve the user's text for clarity,
grammar, and style while preserving the original meaning and tone. Focus on
making the text more professional and polished without adding unnecessary
information."#
        .to_string()
}

pub fn ensure_default_instruction(config_dir: &std::path::Path) -> Result<std::path::PathBuf, InstructionError> {
    let instructions_dir = config_dir.join("instructions");
    fs::create_dir_all(&instructions_dir)?;

    let default_path = instructions_dir.join("default.md");
    if !default_path.exists() {
        fs::write(&default_path, default_instruction())?;
    }

    Ok(default_path)
}
