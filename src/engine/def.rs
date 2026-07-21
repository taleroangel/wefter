use crate::error::WefterErr;
use mlua::{Error, FromLua, Function, Lua, Result, Table, Value};
use std::collections::HashMap;

/// Map with command name and its definition
pub type CommandMap = HashMap<String, CommandDef>;

/// Command structure inside init.lua file
#[derive(Debug)]
pub struct CommandDef {
    /// Command description
    pub description: Option<String>,
    /// Subcommands, with their name and definition
    pub subcommand: Option<CommandMap>,
    /// Execute command function
    pub exec: Option<Function>,
}

impl CommandDef {
    /// Get a list of subcommands as [String]
    pub fn get_subcommands(&self) -> Option<Vec<String>> {
        let subcommand = self.subcommand.as_ref()?;
        Some(subcommand.keys().cloned().collect())
    }
}

impl FromLua for CommandDef {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        let def = value
            .as_table()
            .ok_or_else(|| Error::external(WefterErr::BadProfileDefinition))?;

        // At least one of the following should exist
        if !def.contains_key("exec")? && !def.contains_key("subcommand")? {
            return Err(Error::external(WefterErr::BadProfileDefinition));
        }

        Ok(Self {
            description: def.get::<Option<String>>("description")?,
            exec: def.get("exec")?,
            // Get subcommands from table pairs
            subcommand: def
                .get::<Option<Table>>("subcommand")?
                .map(|t| {
                    let mut cm = CommandMap::new();
                    for pair in t.pairs::<String, Value>() {
                        let (k, v) = pair?;
                        let cdef = CommandDef::from_lua(v, lua)?;
                        cm.insert(k, cdef);
                    }

                    Result::Ok(cm)
                })
                .transpose()?,
        })
    }
}

/// Profile structure returned by init.lua
#[derive(Debug)]
pub struct ProfileDef(pub CommandMap);

impl FromLua for ProfileDef {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        let def = value
            .as_table()
            .ok_or_else(|| Error::external(WefterErr::BadProfileDefinition))?;

        let mut cm = CommandMap::new();
        for pair in def.pairs::<String, Value>() {
            let (k, v) = pair?;
            let cdef = CommandDef::from_lua(v, lua)?;
            cm.insert(k, cdef);
        }

        Ok(Self(cm))
    }
}
