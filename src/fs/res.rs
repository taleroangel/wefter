use super::dir::DirCfg;
use crate::error::LoomErr;
use anyhow::{Ok, Result, anyhow};
use std::collections::hash_map::HashMap;
use std::fs;
use std::path::PathBuf;

/// Name of the lua initial file
const INIT_LUA_FILE: &str = "init.lua";

/// Name of the templates directory within the kind resource
const TEMPLATE_FOLDER_DIR: &str = "templates";

/// Directory containing a `kind resource`.
/// Kind resources are directories that contain an init.lua file
/// and all of its templates
#[derive(Debug)]
pub struct ResourceDir {
    /// Path to the kind directory
    pub path: PathBuf,
    /// init.lua file in resource
    pub luainit: PathBuf,
    /// Templates directory
    pub templates: PathBuf,
}

impl ResourceDir {
    /// Create a [ResourceDir] given its path
    fn new(dir: PathBuf) -> Result<(String, ResourceDir)> {
        // Get path to init.lua
        let mut luainit = dir.clone();
        luainit.push(INIT_LUA_FILE);

        // init.lua is required
        if !luainit.is_file() {
            return Err(LoomErr::InvalidResource(luainit).into());
        }

        // Get path to resources
        let mut templates = dir.clone();
        templates.push(TEMPLATE_FOLDER_DIR);

        // This one is not required, create template if required
        if !templates.is_dir() {
            fs::create_dir(&templates)?;
            log::debug!("Created template directory: {:?}", &templates);
        }

        // Get basename
        let kind = String::from(
            dir
                .file_name()
                .and_then(|os| os.to_str())
                .ok_or(anyhow!("Cannot cast {:?} into String", &dir))?,
        );

        // Build the item
        let item = Self {
            path: dir,
            luainit,
            templates,
        };

        // Append resource
        Ok((kind, item))
    }

    /// Load all the [ResourceDir] paths, key is `kind`
    pub fn load(dirs: &DirCfg) -> Result<HashMap<String, ResourceDir>> {
        // Build the result map
        let mut resources = HashMap::new();

        // Check local directories first
        if let Some(localdir) = &dirs.local {
            for entry in fs::read_dir(localdir)? {
                // Get every folder
                let path = entry?.path();
                if path.is_dir() {
                    // Show resource found
                    log::trace!("Found resource (local): {:?}", &path);

                    // Load resource paths
                    let (kind, res) = Self::new(path)?;
                    log::trace!("Registered '{}' (local): {:?}", &kind, &res);

                    // Store the resource
                    resources.insert(kind, res);
                }
            }
        }

        // Check default resource path
        for entry in fs::read_dir(&dirs.data)? {
            // Get every folder
            let path = entry?.path();
            if path.is_dir() {
                // Show resource found
                log::trace!("Found resource (system): {:?}", &path);

                // Load resource paths
                let (kind, res) = Self::new(path)?;
                log::trace!("Registered kind '{}' (system): {:?}", &kind, &res);

                // Store the resource
                resources.insert(kind, res);
            }
        }

        Ok(resources)
    }
}
