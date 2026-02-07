use anyhow::{Ok, Result};
use clap::Parser;

mod cli;
mod config;
mod error;
mod fs;

/// Wrapper around main to handle error with custom formatting
fn try_main() -> Result<()> {
    // Read command line parameters, setup loggger
    let params = cli::Params::try_parse()?;
    cli::log::setup_logger(params.verbose)?;
    log::trace!("{:?}", &params);

    // Setup base directories
    let mut dirs = fs::dir::DirCfg::new()?;
    dirs.create_if_not_exist()?;

    // Read configuration file
    let cfg = config::CfgFile::read(&dirs)?;
    log::trace!("{:?}", &cfg);

    // Change data directory from config (Partial move of config)
    if let Some(dir) = cfg.data_dir {
        dirs.update_data_dir(dir)?;
    }

    // Change local directory from params (Partial move of params)
    if dirs.update_local_dir(params.local_res_dir).is_ok() {
        log::info!("Reading from project directory: {:?}", &dirs.local);
    }

    // Show final directory structure
    log::trace!("{:?}", &dirs);
    Ok(())
}

fn main() -> () {
    // Terminate with error
    if let Err(e) = try_main() {
        log::error!("{:#}", e);
        std::process::exit(1);
    }
}
