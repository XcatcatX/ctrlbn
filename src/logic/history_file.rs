use std::fs;
use std::path::{Path, PathBuf};

use crate::theme;

// The file lies next to the .exe
pub fn default_path() -> PathBuf {
    let exe_folder = std::env::current_exe()
        .ok()
        .and_then(|exe_path| exe_path.parent().map(Path::to_path_buf))
        .unwrap_or_default();
    exe_folder.join(theme::HISTORY_FILE_NAME)
}

// One entry per line, written as a JSON string so line breaks are safe
pub fn load(path: &Path) -> Vec<String> {
    let Ok(content) = fs::read_to_string(path) else {
        return Vec::new();
    };
    content
        .lines()
        .filter_map(|line| serde_json::from_str::<String>(line).ok())
        .collect()
}

pub fn save(path: &Path, items: &[String]) {
    let content = items
        .iter()
        .filter_map(|item| serde_json::to_string(item).ok())
        .collect::<Vec<_>>()
        .join("\n");
    if let Err(error) = fs::write(path, content) {
        eprintln!("Cannot save file: {error}");
    }
}
