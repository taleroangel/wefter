use crate::fs::dirs::DirCfg;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Configuration filename
const CONFIGURATION_FILENAME: &str = "wefter.toml";

/// App configuration file structure
#[derive(Debug, Deserialize, Serialize)]
pub struct CfgFile {
    /// Directory where profile resources live
    pub data_dir: Option<PathBuf>,
}

impl CfgFile {
    /// Create the default config structure using default config path
    pub fn default(dircfg: &DirCfg) -> Self {
        Self {
            data_dir: Some(dircfg.data.clone()),
        }
    }

    /// Get a [PathBuf] to the default location of the file
    pub fn get_default_path(dircfg: &DirCfg) -> PathBuf {
        let mut cfg = dircfg.cfg.clone();
        cfg.push(CONFIGURATION_FILENAME);
        cfg
    }

    /// Create the configuration file (wefter.toml) if it doesn't exist already
    pub fn create_if_not_exists(dircfg: &DirCfg) -> Result<()> {
        // Get file path
        let fpath = CfgFile::get_default_path(dircfg);

        // Check if file exists
        if !fpath.is_file() {
            // Create default file
            let cfgfile = CfgFile::default(dircfg);
            // Serialize and write
            let ftoml = toml::to_string(&cfgfile)?;
            fs::write(&fpath, ftoml)?;
            log::debug!("Created file: {:?}", fpath);
        }

        Ok(())
    }

    /// Read the configuration file from its default path
    /// Creates the file if it doesn't exist already with the default
    /// directories from [DirCfg]
    pub fn read(dircfg: &DirCfg) -> Result<Self> {
        // Path to the file
        let fpath = Self::get_default_path(dircfg);
        // Create file if it doesn't exist
        if !fpath.is_file() {
            Self::create_if_not_exists(dircfg)?;
        }
        // Read raw text from file
        let fstr = fs::read_to_string(fpath)?;
        // Deserialize into struct
        let cfg = toml::from_str(&fstr)?;
        // Return struct
        Ok(cfg)
    }
}
