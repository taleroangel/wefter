use anyhow::Result;
use std::{fs, path::PathBuf};

/// Get all entries in a directory
pub fn read_directory(path: &PathBuf) -> Result<Vec<PathBuf>> {
    Ok(fs::read_dir(path)?
        .into_iter()
        .flatten()
        .map(|e| e.path())
        .collect())
}
