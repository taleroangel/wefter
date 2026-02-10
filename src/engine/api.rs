use anyhow::{Ok, Result};
use mlua::{Function, Lua};
use std::path::PathBuf;

/// Type of the Loom api table
pub type LoomModuleTable<'a> = Vec<(&'a str, Function)>;

/// Name for the Loom api
pub const LUA_LOOM_TABLE_NAME: &str = "loom";

/// Loom version as constant in lua scripts
pub const LUA_LOOM_VERSION: (&str, &str) = ("LOOM_VERSION", env!("CARGO_PKG_VERSION"));

/// Name for a constant that contains the absolute path to the project root
pub const LUA_LOOM_PROJECT_ROOT: &str = "LOOM_PROJECT_ROOT";

/// Create a table with the 'fs' submodule
pub fn fs_module(l: &Lua) -> Result<LoomModuleTable<'_>> {
    Ok(vec![
        (
            "is_file",
            l.create_function(move |_, path: PathBuf| Result::Ok(path.is_file()))?,
        ),
        (
            "is_dir",
            l.create_function(move |_, path: PathBuf| Result::Ok(path.is_dir()))?,
        ),
    ])
}
