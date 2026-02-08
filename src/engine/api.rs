use anyhow::{Ok, Result, anyhow};
use mlua::{Function, Lua};

use crate::fs::dirs::DirCfg;

/// Type of the Loom api table
pub type LoomTable<'a> = Vec<(&'a str, Function)>;

/// Name for the Loom api
pub const LUA_LOOM_TABLE_NAME: &str = "loom";

/// Loom version as constant in lua scripts
pub const LUA_LOOM_VERSION: (&str, &str) = ("LOOM_VERSION", env!("CARGO_PKG_VERSION"));

/// Name for a constant that contains the absolute path to the project root
pub const LUA_LOOM_PROJECT_ROOT: &str = "LOOM_PROJECT_ROOT";

/// Append function `get_project_root` to [LoomTable]
pub fn register_get_project_root(l: &Lua, api: &mut LoomTable, dirs: &DirCfg) -> Result<()> {
    let project_root = dirs
        .root
        .clone()
        .into_os_string()
        .into_string()
        .map_err(|_| anyhow!("Failed conversion from OsString into String"))?;

    api.push((
        "get_project_root",
        l.create_function(move |_, ()| Result::Ok(project_root.clone()))?,
    ));

    Ok(())
}
