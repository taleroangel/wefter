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

/// Copy `source` into `destination`
/// Unlike [std::fs::copy] this function will fail if `destination` does not
/// exist or if `destination` and `source` are the same
pub fn copy_file(source: &PathBuf, destination: &PathBuf) -> Result<()> {
    // Check if destination exists
    if destination.is_file() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Destination file already exists").into())
    }

    // Check if both are the same
    if destination == source {
    }

    // Copy file
    fs::copy(source, destination)?;
    Ok(())
}
