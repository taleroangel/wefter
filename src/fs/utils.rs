use anyhow::{Ok, Result};
use std::{fs, io, path::PathBuf};

/// Get all entries in a directory
pub fn read_directory(path: &PathBuf) -> Result<Vec<PathBuf>> {
    Ok(fs::read_dir(path)?
        .into_iter()
        .flatten()
        .map(|e| e.path())
        .collect())
}

/// Move a `file` into a new `dir` and return the new [PathBuf] for the file
pub fn move_to_directory(file: &PathBuf, dir: &PathBuf) -> Result<PathBuf> {
    // Check that target directory exists
    // This is important to avoid overwriting other files
    if !dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory,
            "Cannot move file into a non-directory",
        )
        .into());
    }

    // Create new path
    let newfile = dir.join(file);

    // Perform move operation
    fs::rename(file, &newfile)?;
    
    // Return the new path
    Ok(newfile)
}
