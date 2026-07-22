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
                log::debug!("[wefter.fs.mkdir] Directory={:?}", path);
                Ok(match fs::create_dir(&path) {
                    Result::Ok(_) => {
                        // Save action in history
                        history
                            .borrow_mut()
                            .push(HistoryAction::DirectoryCreated(path));
                        Value::Nil
                    }
                    Result::Err(err) => err.to_string().into_lua(lua)?,
                })
            })?
        }),
        ("mkfile", {
            let history = history.clone();
            l.create_function(move |lua, path: PathBuf| {
                log::debug!("[wefter.fs.mkfile] File={:?}", path);
                Ok(match fs::File::create(&path) {
                    Result::Ok(_) => {
                        // Save action in history
                        history.borrow_mut().push(HistoryAction::FileCreated(path));
                        Value::Nil
                    }
                    Result::Err(err) => err.to_string().into_lua(lua)?,
                })
            })?
        }),
        ("rename", {
            let history = history.clone();
            l.create_function(move |lua, (file, newname): (PathBuf, String)| {
                log::debug!("[wefter.fs.rename] File={:?}, Name={}", file, newname);
                let newfile = file.as_path().with_file_name(newname);
                Ok(match fs::rename(&file, &newfile) {
                    Result::Ok(_) => {
                        history.borrow_mut().push(HistoryAction::FileRenamed {
                            previous: file,
                            new: newfile.clone(),
                        });
                        // Return success value and nil for error
                        (newfile.to_path_buf().into_lua(lua)?, Value::Nil)
                    }
                    Result::Err(err) => (Value::Nil, err.to_string().into_lua(lua)?),
                })
            })?
        }),
        ("move", {
            let history = history.clone();
            l.create_function(move |lua, (file, dir): (PathBuf, PathBuf)| {
                log::debug!("[wefter.fs.move] File={:?}, Directory={:?}", file, dir);
                Ok(match wefterfs::utils::move_to_directory(&file, &dir) {
                    Result::Ok(new) => {
                        history.borrow_mut().push(HistoryAction::FileMoved {
                            previous: file,
                            new: new.clone(),
                        });
                        // Return success value and nil for error
                        (new.into_lua(lua)?, Value::Nil)
                    }
                    Result::Err(err) => (Value::Nil, err.to_string().into_lua(lua)?),
                })
            })?
        }),
        ("delete", {
            let history = history.clone();
            l.create_function(move |lua, file: PathBuf| {
                log::debug!("[wefter.fs.delete] File={:?}", file);
                Ok(match fs::remove_file(&file) {
                    Result::Ok(_) => {
                        // Save action in history
                        history.borrow_mut().push(HistoryAction::FileDeleted(file));
                        Value::Nil
                    }
                    Result::Err(err) => err.to_string().into_lua(lua)?,
                })
            })?
        }),
        ("copy", {
            let history = history.clone();
            l.create_function(move |lua, (src, dst): (PathBuf, PathBuf)| {
                log::debug!("[wefter.fs.copy] File={:?}, File={:?}", src, dst);
                Ok(match wefterfs::utils::copy_file(&src, &dst) {
                    Result::Ok(_) => {
                        // Save action in history
                        history
                            .borrow_mut()
                            .push(HistoryAction::FileCreated(dst.clone()));
                        history
                            .borrow_mut()
                            .push(HistoryAction::FileCopied { src: src, dst: dst });
                        Value::Nil
                    }
                    Result::Err(err) => err.to_string().into_lua(lua)?,
                })
            })?
        }),
        /* @wefter.embed:fs */
    ])
}
