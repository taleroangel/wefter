#![allow(dead_code)]
#![allow(unused)]

use anyhow::{Ok, Result};
use clap::{CommandFactory, Parser};
use mlua::Lua;

mod cli;
mod config;
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
        dirs.update_working_dir(wd)?;
    }

    // Read configuration file
    let cfg = config::CfgFile::read(&dirs)?;
    log::trace!("{:?}", &cfg);

    // Override data directory from config (Partial move of config)
    if let Some(dir) = cfg.data_dir {
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

    // List resources & end program
    if params.list {
        tui.print_resources(&resources, &dirs);
        return Ok(());
    }

    // Create lua interpreter
    let lua = Lua::new();

    Ok(())
}

fn main() -> () {
    // Terminate with error
    if let Err(e) = try_main() {
        log::error!("{:#}", e);
        std::process::exit(1);
    }
}
