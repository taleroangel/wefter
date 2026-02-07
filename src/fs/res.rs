use super::dirs::DirCfg;
use crate::error::LoomErr;
use anyhow::{Ok, Result, anyhow};
use std::collections::hash_map::HashMap;
use std::fs;
use std::path::PathBuf;

/// Name of the profile lua initial file
const INIT_LUA_FILE: &str = "init.lua";

/// Name of the autodetect lua function
const AUTODETECT_LUA_FILE: &str = "autodetect.lua";

/// Name of the templates directory within the profile directory
const TEMPLATE_FOLDER_DIR: &str = "templates";

/// Directory structure for a profile.
/// Profiles are directories that contain at least an init.lua file
/// and a template directory
#[derive(Debug)]
pub struct ResourceDir {
    /// Path to the profile directory
    pub path: PathBuf,
    /// init.lua file in resource
    pub luainit: PathBuf,
    /// autodetect.lua file in resource
    pub autodetect: Option<PathBuf>,
    /// Templates directory
    pub templates: PathBuf,
}

/// [ResourceDir] associated with its profile name as [String]
pub type ResourceDirTable = HashMap<String, ResourceDir>;

impl ResourceDir {
    /// Create a [ResourceDir] given its profile path directory
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

        // Get autodetect lua if present
        let mut autodetect = dir.clone();
        autodetect.push(AUTODETECT_LUA_FILE);

        // Get basename
        let profile = String::from(
            dir.file_name()
                .and_then(|os| os.to_str())
                .ok_or(anyhow!("Cannot cast {:?} into String", &dir))?,
        );

        // Build the item
        let item = Self {
            path: dir,
            luainit,
            templates,
            // Only if autodetect.lua exists
            autodetect: if autodetect.is_file() {
                Some(autodetect)
            } else {
                None
            },
        };

        // Append resource
        Ok((profile, item))
    }

    /// Load all the profiles found in resource directories
    pub fn load(dirs: &DirCfg) -> Result<ResourceDirTable> {
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
                    let (profile, res) = Self::new(path)?;
                    log::trace!("Registered profile '{}' (local): {:?}", &profile, &res);

                    // Store the resource
                    resources.insert(profile, res);
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
                let (profile, res) = Self::new(path)?;
                log::trace!("Registered profile '{}' (system): {:?}", &profile, &res);

                // Store the resource
                resources.insert(profile, res);
            }
        }

        Ok(resources)
    }
}
