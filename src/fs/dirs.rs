use crate::error::LoomErr;
use anyhow::Result;
use directories::ProjectDirs;
use std::{env, fs, path::PathBuf};

const APP_QUALIFIER: &str = env!("LOOM_PRJ_QUALIFIER");
const APP_ORGANIZATION: &str = env!("LOOM_PRJ_ORG");
const APP_NAME: &str = env!("LOOM_PRJ_NAME");

/// Directories configuration, contains paths to all the directories
/// loom requires to load templates
#[derive(Debug)]
pub struct DirCfg {
    /// Project working directory
    pub root: PathBuf,
    /// Directory for app configurations ($HOME/.config/loom)
    pub cfg: PathBuf,
    /// Directory for configurations & templates ($HOME/.local/share/loom)
    pub data: PathBuf,
    /// Directory for (project) local data directory
    pub local: Option<PathBuf>,
}

impl DirCfg {
    /// Create initial directory configuration
    pub fn new() -> Result<Self> {
        // Get project directories (OS-agnostic)
        let prjdir = ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
            .ok_or(LoomErr::FilesystemError)?;

        Ok(Self {
            root: std::env::current_dir()?,
            cfg: prjdir.config_local_dir().to_path_buf(),
            data: prjdir.data_local_dir().to_path_buf(),
            local: None,
        })
    }

    /// Create data & config directories if they dont exist
    pub fn create_if_not_exist(&self) -> Result<()> {
        // Create config directory
        if !self.cfg.is_dir() {
            fs::create_dir_all(&self.cfg)?;
            log::debug!("Created directory: {:?}", &self.cfg);
        }

        // Create data directory
        if !self.data.is_dir() {
            fs::create_dir_all(&self.data)?;
            log::debug!("Created directory: {:?}", &self.data);
        }

        Ok(())
    }

    /// Update current working directory location
    pub fn update_working_dir(&mut self, newdir: PathBuf) -> Result<()> {
        if !newdir.is_dir() {
            return Err(LoomErr::BadRootDirectory(newdir).into());
        }

        env::set_current_dir(&newdir)?;
        self.root = newdir;
        Ok(())
    }

    /// Change the data directory
    pub fn update_data_dir(&mut self, newdir: PathBuf) -> Result<()> {
        if !newdir.is_dir() {
            return Err(LoomErr::NoSuchResourceDirectory(newdir).into());
        }

        self.data = newdir;
        Ok(())
    }

    /// Change the local directory, usually provided by cfg
    pub fn update_local_dir(&mut self, newdir: PathBuf) -> Result<()> {
        // Check absolute path
        if newdir.is_dir() {
            self.local = Some(newdir);
            return Ok(());
        }

        // Build relative path (from cwd)
        let mut wd = self.root.clone();
        wd.push(newdir);

        // Check relative path
        if wd.is_dir() {
            self.local = Some(wd);
            return Ok(());
        }

        // Path doesn't exist
        Err(LoomErr::NoSuchResourceDirectory(wd).into())
    }
}
