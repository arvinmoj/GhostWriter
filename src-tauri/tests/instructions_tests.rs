use std::fs;
use tempfile::TempDir;

use ghostwriter_lib::instructions::{
    create_sample_instructions, default_instruction, ensure_default_instruction, load_instruction,
    InstructionError,
};

fn temp_file(content: &str) -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.md");
    fs::write(&path, content).unwrap();
    (dir, path)
}

#[test]
fn test_load_valid_file() {
    let (_dir, path) = temp_file("Fix grammar and spelling.");
    let result = load_instruction(&path).unwrap();
    assert_eq!(result, "Fix grammar and spelling.");
}

#[test]
fn test_load_nonexistent_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nonexistent.md");
    let result = load_instruction(&path);
    assert!(result.is_err());
    match result {
        Err(InstructionError::FileNotFound(_)) => {}
        _ => panic!("Expected FileNotFound"),
    }
}

#[test]
fn test_load_empty_file() {
    let (_dir, path) = temp_file("   \n  \t  ");
    let result = load_instruction(&path);
    assert!(result.is_err());
    match result {
        Err(InstructionError::EmptyFile(_)) => {}
        _ => panic!("Expected EmptyFile"),
    }
}

#[test]
fn test_load_whitespace_only_file() {
    let (_dir, path) = temp_file("");
    let result = load_instruction(&path);
    assert!(result.is_err());
    match result {
        Err(InstructionError::EmptyFile(_)) => {}
        _ => panic!("Expected EmptyFile"),
    }
}

#[test]
fn test_default_instruction_content() {
    let content = default_instruction();
    assert!(!content.is_empty());
    assert!(content.to_lowercase().contains("improve"));
    assert!(content.to_lowercase().contains("text"));
}

#[test]
fn test_ensure_default_creates_file() {
    let dir = TempDir::new().unwrap();
    let path = ensure_default_instruction(dir.path()).unwrap();
    assert!(path.exists());
    let content = fs::read_to_string(&path).unwrap();
    assert!(!content.is_empty());
}

#[test]
fn test_ensure_default_does_not_overwrite() {
    let dir = TempDir::new().unwrap();
    let instructions_dir = dir.path().join("instructions");
    fs::create_dir_all(&instructions_dir).unwrap();
    let default_path = instructions_dir.join("default.md");
    fs::write(&default_path, "CUSTOM CONTENT").unwrap();

    let path = ensure_default_instruction(dir.path()).unwrap();
    let content = fs::read_to_string(&path).unwrap();
    assert_eq!(content, "CUSTOM CONTENT");
}

#[test]
fn test_create_sample_instructions() {
    let dir = TempDir::new().unwrap();
    create_sample_instructions(dir.path()).unwrap();

    let grammar = dir.path().join("instructions").join("grammar.md");
    let translate = dir
        .path()
        .join("instructions")
        .join("translate_to_french.md");
    let friendly = dir.path().join("instructions").join("friendly.md");
    let concise = dir.path().join("instructions").join("concise.md");

    assert!(grammar.exists());
    assert!(translate.exists());
    assert!(friendly.exists());
    assert!(concise.exists());

    let grammar_content = fs::read_to_string(&grammar).unwrap();
    assert!(grammar_content.contains("professional editor"));
}

#[test]
fn test_create_sample_instructions_idempotent() {
    let dir = TempDir::new().unwrap();
    create_sample_instructions(dir.path()).unwrap();
    create_sample_instructions(dir.path()).unwrap();
}

#[test]
fn test_error_display_messages() {
    let err = InstructionError::FileNotFound("/tmp/test.md".to_string());
    assert_eq!(err.to_string(), "Instruction file not found: /tmp/test.md");

    let err = InstructionError::EmptyFile("/tmp/empty.md".to_string());
    assert_eq!(err.to_string(), "Instruction file is empty: /tmp/empty.md");
}
