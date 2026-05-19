use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

pub fn temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

pub fn temp_file(content: &str) -> (TempDir, PathBuf) {
    let dir = temp_dir();
    let path = dir.path().join("test.md");
    fs::write(&path, content).expect("Failed to write temp file");
    (dir, path)
}

pub fn temp_dir_with_file(name: &str, content: &str) -> (TempDir, PathBuf) {
    let dir = temp_dir();
    let path = dir.path().join(name);
    fs::write(&path, content).expect("Failed to write temp file");
    (dir, path)
}
