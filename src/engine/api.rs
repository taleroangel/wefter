use crate::fs as loomfs;
use anyhow::Result;
use mlua::{Function, IntoLua, Lua, Value};
use std::{fs, path::PathBuf};

/// Type of the Loom api table
pub type LoomModuleTable<'a> = Vec<(&'a str, Function)>;

/// Name for the Loom api
pub const LUA_LOOM_TABLE_NAME: &str = "loom";

/// Loom version as constant in lua scripts
pub const LUA_LOOM_VERSION: (&str, &str) = ("LOOM_VERSION", env!("CARGO_PKG_VERSION"));

/// Name for a constant that contains the absolute path to the project root
pub const LUA_LOOM_PROJECT_ROOT: &str = "LOOM_PROJECT_ROOT";

/// Wrap a [Result<T, E>] into an standard Lua error tuple (T::IntoLua, String).
///
/// errors int the original [Result] are converted to string into the second
/// tuple field, errors during conversion are returned within the [Result]
/// of this function
fn wrap_lua_error<T: IntoLua, E: ToString>(
    lua: &Lua,
    res: Result<T, E>,
) -> Result<(Value, Value), mlua::Error> {
    Ok(match res {
        Result::Ok(value) => (value.into_lua(lua)?, Value::Nil),
        Result::Err(err) => (Value::Nil, err.to_string().into_lua(lua)?),
    })
}

/// Create a table with the 'fs' submodule
pub fn fs_module(l: &Lua) -> Result<LoomModuleTable<'_>> {
    Ok(vec![
        // Check if a path exists and is a regular file
        (
            "is_file",
            l.create_function(move |_, path: PathBuf| Result::Ok(path.is_file()))?,
        ),
        // Check if a path exists and is a directory
        (
            "is_dir",
            l.create_function(move |_, path: PathBuf| Result::Ok(path.is_dir()))?,
        ),
        // Read file contents into a string
        (
            "read_to_string",
            l.create_function(move |lua, path: PathBuf| {
                wrap_lua_error(lua, fs::read_to_string(path))
            })?,
        ),
        // List all files in a directory
        (
            "read_dir",
            l.create_function(move |lua, path: PathBuf| {
                wrap_lua_error(lua, loomfs::utils::read_directory(&path))
            })?,
        ),
    ])
}
