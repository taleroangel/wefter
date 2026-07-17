use super::*;
use crate::fs as loomfs;
use std::{fs, path::PathBuf};

/// Create a table for the 'fs' submodule
pub fn module(l: &Lua) -> Result<LoomModuleTable<'_>> {
    Ok(vec![
        // Check if a path exists and is a regular file
        (
            "is_file",
            l.create_function(|_, path: PathBuf| {
                log::debug!("[loom.fs.is_file] File={:?}", path);
                Result::Ok(path.is_file())
            })?,
        ),
        // Check if a path exists and is a directory
        (
            "is_dir",
            l.create_function(|_, path: PathBuf| {
                log::debug!("[loom.fs.is_dir] Directory={:?}", path);
                Result::Ok(path.is_dir())
            })?,
        ),
        // Read file contents into a string
        (
            "read_to_string",
            l.create_function(|lua, path: PathBuf| {
                log::debug!("[loom.fs.read_to_string] File={:?}", path);
                wrap_error_tuple(lua, fs::read_to_string(path))
            })?,
        ),
        // List all files in a directory
        (
            "read_dir",
            l.create_function(|lua, path: PathBuf| {
                log::debug!("[loom.fs.read_dir] Directory={:?}", path);
                wrap_error_tuple(lua, loomfs::utils::read_directory(&path))
            })?,
        ),
        /* @loom.embed:fs */
    ])
}
