use super::api;
use crate::{
    error::LoomErr,
    fs::{
        dirs::DirCfg,
        res::{ResourceDir, ResourceDirTable},
    },
};
use anyhow::{Ok, Result};
use mlua::{FromLua, Lua};
use std::{fs, path::PathBuf};

/// Wrapper for the Lua interpreter and the variables it need to load
pub struct LuaInterpreter {
    interpreter: Lua,
}

// Private
impl LuaInterpreter {
    /// Execute a single file as it were a function
    fn exec<T: FromLua>(&mut self, path: &PathBuf) -> Result<T> {
        if !path.is_file() {
            return Err(LoomErr::NoSuchLuaFile(path.clone()).into());
        }
        let file = fs::read_to_string(&path)?;
        let chunk = self.interpreter.load(file);
        let result = chunk.call::<T>(())?;
        Ok(result)
    }
}

// Public
impl LuaInterpreter {
    /// Create an instance of the interpreter
    pub fn new(dirs: &DirCfg) -> Result<Self> {
        let interpreter = Lua::new();
        let globals = interpreter.globals();

        globals.set(api::LUA_LOOM_VERSION.0, api::LUA_LOOM_VERSION.1)?;
        globals.set(api::LUA_LOOM_PROJECT_ROOT, dirs.root.clone())?;

        Ok(Self { interpreter })
    }

    /// Run all the registered autodetect functions to tell which profiles
    /// can be activated, returns profiles keys
    pub fn run_autodetect(&mut self, res: &ResourceDirTable) -> Result<Vec<String>> {
        res.iter()
            // Get only ones with 'autodetect', keep only the path
            .filter_map(|(k, v)| v.autodetect.clone().map(|e| (k.clone(), e)))
            // Execute each autodetect.lua file
            .map(|(k, p)| (k, self.exec::<bool>(&p)))
            // Iter<K, Result<R, Err>> -> Iter<Result<(K, R), Err>>
            .filter_map(|(k, r)| match r {
                Result::Ok(true) => Some(Ok(k)),
                Result::Ok(false) => None,
                Result::Err(e) => Some(Err(e)),
            })
            .collect()
    }

    /// Run a configuration file
    pub fn run_init(
        &mut self,
        params: Vec<String>,
        res: &ResourceDir,
        dirs: &DirCfg,
    ) -> Result<()> {
        // Check if file exists
        if !res.luainit.is_file() {
            return Err(LoomErr::NoSuchLuaFile(res.luainit.clone()).into());
        }

        // In this table, all APi functions are registered
        let mut table = api::LoomTable::new();

        // Reference to the interpreter just for simplicity
        let l = &mut self.interpreter;

        // Register all functions
        api::register_get_project_root(&l, &mut table, &dirs)?;

        // Build and insert the API as a table
        let loom = l.create_table_from(table)?;
        l.globals().set(api::LUA_LOOM_TABLE_NAME, loom)?;

        Ok(())
    }
}
