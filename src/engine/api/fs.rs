use super::*;
use crate::fs::{
    self as wefterfs,
    hist::{HistoryAction, HistoryRef},
};
use std::{fs, path::PathBuf};

/// Create a table for the 'fs' submodule
pub fn module(l: &Lua, history: HistoryRef) -> Result<WefterModuleTable<'_>> {
    Ok(vec![
        // Check if a path exists and is a regular file
        (
            "is_file",
            l.create_function(|_, path: PathBuf| {
                log::debug!("[wefter.fs.is_file] File={:?}", path);
                Result::Ok(path.is_file())
            })?,
        ),
        // Check if a path exists and is a directory
        (
            "is_dir",
            l.create_function(|_, path: PathBuf| {
                log::debug!("[wefter.fs.is_dir] Directory={:?}", path);
                Result::Ok(path.is_dir())
            })?,
        ),
        // Read file contents into a string
        (
            "read_to_string",
            l.create_function(|lua, path: PathBuf| {
                log::debug!("[wefter.fs.read_to_string] File={:?}", path);
                wrap_error_tuple(lua, fs::read_to_string(path))
            })?,
        ),
        // List all files in a directory
        (
            "read_dir",
            l.create_function(|lua, path: PathBuf| {
                log::debug!("[wefter.fs.read_dir] Directory={:?}", path);
                wrap_error_tuple(lua, wefterfs::utils::read_directory(&path))
            })?,
        ),
        ("mkdir", {
            let history = history.clone();
            l.create_function(move |lua, path: PathBuf| {
                Ok(match fs::create_dir(&path) {
                    Result::Ok(_) => {
                        // Save action in history
                        history
                            .borrow_mut()
                            .push(HistoryAction::CreateDirectory(path));
                        Value::Nil 
                    }
                    Result::Err(err) => err.to_string().into_lua(lua)?,
                })
            })?
        }),
        ("mkfile", {
            let history = history.clone();
            l.create_function(move |lua, path: PathBuf| {
                Ok(match fs::File::create(&path) {
                    Result::Ok(_) => {
                        // Save action in history
                        history
                            .borrow_mut()
                            .push(HistoryAction::CreateFile(path));
                        Value::Nil
                    }
                    Result::Err(err) => err.to_string().into_lua(lua)?,
                })
            })?
        }),
        /* @wefter.embed:fs */
    ])
}
