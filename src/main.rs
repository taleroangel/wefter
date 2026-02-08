#![allow(dead_code)]
#![allow(unused)]

use crate::{error::LoomErr, fs::res::ResourceDir};
use anyhow::{Ok, Result};
use clap::{CommandFactory, Parser};

mod cli;
mod config;
mod engine;
mod error;
mod fs;
mod tui;

/// Wrapper around main to handle errors with custom formatting
fn try_main() -> Result<()> {
    let tui = tui::TuiInterface::new();

    // Parse command line arguments
    let params = cli::Params::parse();
    if params.help {
        tui.print_help();
        return Ok(());
    }

    // Setup logger
    cli::setup_logger(params.verbose)?;
    log::trace!("{:?}", &params);

    // Setup base directories
    let mut dirs = fs::dirs::DirCfg::new()?;
    dirs.create_if_not_exist()?;

    // Override cwd (Partial move of params)
    if let Some(wd) = params.root {
        log::info!("Changed working directory to: {:?}", &wd);
        dirs.update_working_dir(wd)?;
    }

    // Read configuration file
    let cfg = config::CfgFile::read(&dirs)?;
    log::trace!("{:?}", &cfg);

    // Override data directory from config (Partial move of config)
    if let Some(dir) = cfg.data_dir {
        log::info!("Configuration set resource directory to: {:?}", &dir);
        dirs.update_data_dir(dir)?;
    }

    // Override local resources directory
    if dirs.update_local_dir(params.local_resources).is_ok() {
        log::info!("Reading from project directory: {:?}", &dirs.local);
    }

    // Show final directory structure
    log::trace!("{:?}", &dirs);

    // Load resources paths
    let resources = fs::res::ResourceDir::load(&dirs)?;
    if resources.is_empty() {
        // TODO: Render some documentation
        return Err(LoomErr::NoAvailableProfiles.into());
    }

    // List resources & end program
    if params.list {
        tui.print_resources(&resources, &dirs);
        return Ok(());
    }

    // Create lua interpreter
    let mut lua = engine::LuaEngine::new();

    // Get the profile directory
    let profile: (&String, &ResourceDir) = if let Some(key) = &params.profile {
        // Get only by name
        resources
            .get_key_value(key)
            .ok_or_else(|| LoomErr::UnknownProfile(key.clone()))
    } else {
        // Uset autodetect.lua
        let autodetect = lua.run_autodetect(&resources)?;
        log::trace!("Valid autodetect profiles: {:?}", &autodetect);

        match autodetect.len() {
            0 => Result::Err(LoomErr::NoProfileSpecified.into()),
            1 => autodetect
                .first()
                // Error for when no items are available
                .ok_or_else(|| LoomErr::NoAvailableProfiles)
                .map(|k| {
                    // Get resource by key
                    resources
                        .get_key_value(k)
                        .ok_or_else(|| LoomErr::UnknownProfile(k.clone()))
                })
                .flatten(),
            1.. => todo!("make user select from list"),
        }
    }?;

    log::debug!("Using profile: {:?}", &profile);

    Ok(())
}

fn main() -> () {
    // Terminate with error
    if let Err(e) = try_main() {
        log::error!("{:#}", e);
        std::process::exit(1);
    }
}
