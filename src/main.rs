use crate::{error::WefterErr, fs::res::ResourceDir};
use anyhow::Result;
use clap::Parser;
use std::rc::Rc;

mod cli;
mod config;
mod engine;
mod error;
mod fs;
mod templates;
mod tui;

/// Wrapper around main to handle errors with custom formatting
fn try_main() -> Result<()> {
    let ui = Rc::new(tui::TuiInterface::new());

    // Parse command line arguments
    let params = cli::Params::parse();
    if params.help {
        ui.print_help();
        return Ok(());
    }

    // Print meta
    if params.meta {
        ui.print_lua_meta();
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
        ui.print_err_no_available_profiles(&dirs);
        return Err(WefterErr::NoAvailableProfiles.into());
    }

    // List resources & end program
    if params.profiles {
        ui.print_profile_list(&resources, &dirs);
        return Ok(());
    }

    // Create lua interpreter
    let mut lua = engine::LuaInterpreter::new(&dirs)?;

    // Get the profile directory
    let profile: (&String, &ResourceDir) = if let Some(key) = &params.profile {
        // Get only by name
        resources
            .get_key_value(key)
            .ok_or_else(|| WefterErr::UnknownProfile(key.clone()))
    } else {
        // Use auto.lua
        let auto = lua.run_auto(&resources)?;
        log::trace!("Valid auto profiles: {:?}", &auto);

        match auto.len() {
            // Show error
            0 => Result::Err(WefterErr::NoProfileSpecified.into()),
            // Use the only entry available
            1 => auto
                .first()
                // Error for when no items are available
                .ok_or_else(|| WefterErr::NoAvailableProfiles)
                .map(|k| {
                    // Get resource by key
                    resources
                        .get_key_value(k)
                        .ok_or_else(|| WefterErr::UnknownProfile(k.clone()))
                })
                .flatten(),
            // Prompt use to choose
            1.. => ui.select("Select a profile", &auto).map(|key| {
                resources
                    .get_key_value(&key)
                    .ok_or_else(|| WefterErr::UnknownProfile(key))
            })?,
        }
    }?;

    log::debug!("Using profile: {:?}", &profile);

    // Initialize the wefter API once the profile has been loaded
    lua.init(&profile.1, ui.clone())?;

    // Load profile definition
    let pdef: engine::ProfileDef = lua.run_init(&profile.1)?;
    log::info!("Successfully loaded profile definition: {:?}", profile.0);
    log::trace!("{:?}", pdef);

    // List profile commands
    if params.commands {
        ui.print_profile(profile.0, &pdef);
        return Ok(());
    }

    // Execute command
    match lua.exec_command(params.trailing, &pdef) {
        Ok(history) => {
            log::debug!("Executed 'init.lua' successfully");
            if !history.borrow().is_empty() {
                ui.print_history(
                    Rc::into_inner(history)
                    .ok_or(WefterErr::ApplicationError(
                            "Failed to take ownership of 'history'".to_string(),
                    ))?
                    .into_inner(),
                );
            }
        }
        Err(err) => match err {
            WefterErr::EmptyParameters => ui.print_err_empty_parameters(profile.0, &pdef),
            _ => ui.print_error(&err),
        },
    };

    Ok(())
}

fn main() -> () {
    // Terminate with error
    if let Err(e) = try_main() {
        log::error!("{:#}", e);
        std::process::exit(1);
    }
}
