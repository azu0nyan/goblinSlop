use std::path::Path;

/// Scan a directory for non-default `.jpg` filenames, sorted lexicographically.
/// Returns an empty vec if the directory cannot be read.
pub fn scan(dir: impl AsRef<Path>) -> Vec<String> {
    let mut images: Vec<String> = std::fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|name| name.ends_with(".jpg") && !name.starts_with("default"))
        .collect();
    images.sort();
    images
}
