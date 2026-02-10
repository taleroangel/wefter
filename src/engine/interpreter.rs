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

    /// Register loader for the init.lua source directory
    fn register_loader(&mut self, profile: &ResourceDir) -> Result<()> {
        let path = profile.path.clone();
        let globals = self.interpreter.globals();
        let package: mlua::Table = globals.get("package")?;
        let searchers: mlua::Table = package.get("searchers")?;

        let loader = self.interpreter.create_function(move |lua, name: String| {
            let mut path = path.clone();
            let name = name.replace(".", "/") + ".lua";
            path.push(&name);

            if !path.is_file() {
                return Result::Err(mlua::Error::runtime(format!(
                    "Could not find module {:?}",
                    path
                )));
            }

            let source = fs::read_to_string(path)?;
            let module = lua.load(source).set_name(name).into_function()?;

            Result::Ok(mlua::Value::Function(module))
        })?;

        searchers.raw_insert(1, loader)?;
        Ok(())
    }
}

// Public
impl LuaInterpreter {
    /// Create an instance of the interpreter and register the Loom API module
    pub fn new(dirs: &DirCfg) -> Result<Self> {
        let l = Lua::new();
        let globals = l.globals();

        // Set global variables
        globals.set(api::LUA_LOOM_VERSION.0, api::LUA_LOOM_VERSION.1)?;
        globals.set(api::LUA_LOOM_PROJECT_ROOT, dirs.root.clone())?;

        // Register all APIs
        let fs = l.create_table_from(api::fs_module(&l)?)?;

        // Create global api and register it
        let loom = l.create_table_from(vec![("fs", fs)])?;
        l.globals().set(api::LUA_LOOM_TABLE_NAME, loom)?;

        Ok(Self { interpreter: l })
    }

    /// Run all the registered auto functions to tell which profiles
    /// can be activated, returns profiles keys
    pub fn run_auto(&mut self, res: &ResourceDirTable) -> Result<Vec<String>> {
        res.iter()
            // Get only ones with 'auto', keep only the path
            .filter_map(|(k, v)| v.auto.clone().map(|e| (k.clone(), e)))
            // Execute each auto.lua file
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
    pub fn run_init(&mut self, params: Vec<String>, res: &ResourceDir) -> Result<()> {
        // Check if file exists
        if !res.init.is_file() {
            return Err(LoomErr::NoSuchLuaFile(res.init.clone()).into());
        }

        // Register the loader for init.lua parent directory
        self.register_loader(res)?;

        let result = self.exec::<String>(&res.init)?;

        Ok(())
    }
}
