use anyhow::{Ok, Result};
use clap::Parser;

mod cli;
mod config;

fn main() -> Result<()> {

    // Read command line parameters, setup loggger
    let params = cli::Params::try_parse()?;
    cli::log::setup_logger(params.verbose)?;
    log::debug!("{:?}", params);

    // Read configuration file

    Ok(())
}
