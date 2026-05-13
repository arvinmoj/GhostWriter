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

pub fn create_sample_instructions(config_dir: &std::path::Path) -> Result<(), InstructionError> {
    let instructions_dir = config_dir.join("instructions");
    fs::create_dir_all(&instructions_dir)?;

    let grammar_path = instructions_dir.join("grammar.md");
    if !grammar_path.exists() {
        fs::write(&grammar_path, r#"You are a professional editor. Correct all grammar,
spelling, and punctuation errors. Preserve the original tone and style.
Do not add or remove information."#)?;
    }

    let translate_path = instructions_dir.join("translate_to_french.md");
    if !translate_path.exists() {
        fs::write(&translate_path, r#"Translate the following text to French.
Keep technical terms and proper nouns in their original English form.
Maintain the original tone and formatting."#)?;
    }

    let friendly_path = instructions_dir.join("friendly.md");
    if !friendly_path.exists() {
        fs::write(&friendly_path, r#"Rewrite the following text in a warm, friendly,
and conversational tone. Make it feel natural and approachable while
preserving the core message."#)?;
    }

    let concise_path = instructions_dir.join("concise.md");
    if !concise_path.exists() {
        fs::write(&concise_path, r#"Shorten the following text to half its length
without losing the essential information. Be direct and concise."#)?;
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum InstructionError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Instruction file not found: {0}")]
    FileNotFound(String),
    #[error("Instruction file is empty: {0}")]
    EmptyFile(String),
}
