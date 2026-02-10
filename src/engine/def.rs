use crate::error::LoomErr;
use mlua::*;
use std::collections::HashMap;

/// Map with command name and its definition
type CommandMap = HashMap<String, CommandDef>;

/// Command structure inside init.lua file
#[derive(Debug)]
pub struct CommandDef {
    /// Subcommands, with their name and definition
    pub subcommand: Option<CommandMap>,

    /// Execute command function
    pub exec: Option<Function>,
}

impl FromLua for CommandDef {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        let def = value
            .as_table()
            .ok_or_else(|| Error::external(LoomErr::BadProfileDefinition))?;

        // At least one of the following should exist
        if !def.contains_key("exec")? && !def.contains_key("subcommand")? {
            return Err(Error::external(LoomErr::BadProfileDefinition));
        }

        Ok(Self {
            // Get exec function
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
pub struct ProfileDef(CommandMap);

impl FromLua for ProfileDef {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        let def = value
            .as_table()
            .ok_or_else(|| Error::external(LoomErr::BadProfileDefinition))?;

        let mut cm = CommandMap::new();
        for pair in def.pairs::<String, Value>() {
            let (k, v) = pair?;
            let cdef = CommandDef::from_lua(v, lua)?;
            cm.insert(k, cdef);
        }

        Ok(Self(cm))
    }
}
