use anyhow::{Ok, Result};
use mlua::{FromLua, Lua};
use std::{fs, path::PathBuf};

use crate::{
    error::LoomErr,
    fs::res::{ResourceDir, ResourceDirTable},
};

/// Loom version to use as a constant to lua scripts
const LUA_LOOM_VERSION: (&str, &str) = ("LOOM_VERSION", env!("CARGO_PKG_VERSION"));

pub struct LuaEngine {
    interpreter: Lua,
}

impl Default for LuaEngine {
    /// Create an instance of the Lua interpreter and load all the libraries
    fn default() -> Self {
        let interpreter = Lua::new();
        interpreter
            .globals()
            .set(LUA_LOOM_VERSION.0, LUA_LOOM_VERSION.1);

        Self { interpreter }
    }
}

impl LuaEngine {
    /// Alias for ::default()
    pub fn new() -> Self {
        Self::default()
    }

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
}
