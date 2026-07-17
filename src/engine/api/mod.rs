mod fs;
mod io;
mod template;

// Common imports between modules
use anyhow::Result;
use mlua::{Function, IntoLua, Lua, LuaSerdeExt, Table, Value};

/// Type of the Loom api table
pub type LoomModuleTable<'a> = Vec<(&'a str, Function)>;

/// Name for the Loom api
pub const LUA_LOOM_TABLE_NAME: &str = "loom";

/// Loom version as constant in lua scripts
pub const LUA_LOOM_VERSION: (&str, &str) = ("LOOM_VERSION", env!("CARGO_PKG_VERSION"));

/// Name for a constant that contains the absolute path to the project root
pub const LUA_LOOM_PROJECT_ROOT: &str = "LOOM_PROJECT_ROOT";

/* Re-export modules */
pub use fs::module as fs_module;
pub use io::module as io_module;
pub use template::module as template_module;

/* PRIVATE FUNCTIONS (Utilities for submodules) */

/// Wrap a [Result<T, E>] into an standard Lua error tuple (T::IntoLua, String).
///
/// errors int the original [Result] are converted to string into the second
/// tuple field, errors during conversion are returned within the [Result]
/// of this function
fn wrap_error_tuple<T: IntoLua, E: ToString>(
    lua: &Lua,
    res: Result<T, E>,
) -> Result<(Value, Value), mlua::Error> {
    Ok(match res {
        Result::Ok(value) => (value.into_lua(lua)?, Value::Nil),
        Result::Err(err) => (Value::Nil, err.to_string().into_lua(lua)?),
    })
}

/// Serialize a [Table] into a json ([serde_json::Value])
pub fn serialize_table(lua: &Lua, table: Table) -> Result<serde_json::Value> {
    let value: Value = Value::Table(table);
    let json: serde_json::Value = lua.from_value(value)?;
    Ok(json)
}
