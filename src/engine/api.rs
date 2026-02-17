use crate::{
    fs::{self as loomfs, res::ResourceDir},
    templates,
    tui::TuiInterface,
};
use anyhow::Result;
use mlua::{Function, IntoLua, Lua, LuaSerdeExt, Table, Value};
use std::{fs, path::PathBuf, rc::Rc};

/// Type of the Loom api table
pub type LoomModuleTable<'a> = Vec<(&'a str, Function)>;

/// Name for the Loom api
pub const LUA_LOOM_TABLE_NAME: &str = "loom";

/// Loom version as constant in lua scripts
pub const LUA_LOOM_VERSION: (&str, &str) = ("LOOM_VERSION", env!("CARGO_PKG_VERSION"));

/// Name for a constant that contains the absolute path to the project root
pub const LUA_LOOM_PROJECT_ROOT: &str = "LOOM_PROJECT_ROOT";

/// String for embedding into files
pub const LUA_LOOM_TEMPLATE_EMBEDDING_POINT: &str = "@loom.embed";

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

/// Create a table for the 'fs' submodule
pub fn fs_module(l: &Lua) -> Result<LoomModuleTable<'_>> {
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

/// Create a table for the 'io' submodule
pub fn io_module(l: &Lua, tui: Rc<TuiInterface>) -> Result<LoomModuleTable<'_>> {
    Ok(vec![
        // Prompt user to input a string
        ("input", {
            let tui = tui.clone();
            l.create_function(move |_, prompt: String| Ok(tui.input(prompt)?))?
        }),
        // Prompt user to choose from a selection
        ("select", {
            let tui = tui.clone();
            l.create_function(move |_, (prompt, opts): (String, Vec<String>)| {
                Ok(tui.select(&prompt, &opts)?)
            })?
        }),
        ("markdown", {
            let tui = tui.clone();
            l.create_function(move |_, content: String| {
                Ok(tui.print_markdown(content))
            })?
        }),
        /* @loom.embed:io */
    ])
}

/// Serialize a [Table] into a json ([serde_json::Value])
pub fn serialize_table(lua: &Lua, table: Table) -> Result<serde_json::Value> {
    let value: Value = Value::Table(table);
    let json: serde_json::Value = lua.from_value(value)?;
    Ok(json)
}

// Create a table for the 'template' submodule
pub fn template_module(l: &Lua, profile: ResourceDir) -> Result<LoomModuleTable<'_>> {
    Ok(vec![
        ("create", {
            let profile = profile.clone();
            l.create_function(
                move |lua, (dst, template, params): (PathBuf, PathBuf, Table)| {
                    let template = profile.build_template_path(template)?;
                    let params = serialize_table(lua, params)?;
                    log::debug!(
                        "[loom.template.create] Creating file {:?} with template {:?}",
                        dst,
                        template
                    );

                    let rendered = templates::render(template, params)?;
                    fs::write(dst, rendered)?;
                    Ok(())
                },
            )?
        }),
        ("embed", {
            let profile = profile.clone();
            l.create_function(
                move |lua,
                      (dst, ipoint, template, params): (
                    PathBuf,
                    Option<String>,
                    PathBuf,
                    Table,
                )| {
                    // Insertion point builder
                    let lookup = match ipoint {
                        Some(e) => format!("{}:{}", LUA_LOOM_TEMPLATE_EMBEDDING_POINT, e),
                        None => format!("{}", LUA_LOOM_TEMPLATE_EMBEDDING_POINT),
                    };
                    let template = profile.build_template_path(template)?;
                    let params = serialize_table(lua, params)?;

                    log::debug!(
                        "[loom.template.embed] template {:?} into {:?} at {:?}",
                        template,
                        dst,
                        lookup
                    );
                    templates::embed(dst, lookup, template, params)?;
                    Ok(())
                },
            )?
        }),
        ("get", {
            let profile = profile.clone();
            l.create_function(move |lua, (template, params): (PathBuf, Table)| {
                let template = profile.build_template_path(template)?;
                let params = serialize_table(lua, params)?;
                
                log::debug!("[loom.template.get] template {:?}", template);
                let rendered = templates::render(template, params)?;
                Ok(rendered)
            })?
        }),
        /* @loom.embed:template */
    ])
}
