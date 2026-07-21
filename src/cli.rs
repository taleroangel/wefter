use anyhow::Result;
use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use std::path::PathBuf;

/// Setup logging interface
pub fn setup_logger(verbose: u8) -> Result<()> {
    // Create colors for level
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Cyan)
        .debug(Color::Magenta)
        .trace(Color::White);

    // Define Ferm logger
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}]\t{}",
                colors.color(record.level()),
                message
            ));
        })
        .level(match verbose {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            3.. => log::LevelFilter::Trace,
        })
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

/// Input parameters for the program
#[derive(Debug, Parser)]
#[command(version, author, disable_help_flag = true, trailing_var_arg = true)]
pub struct Params {
    /// Print Help
    #[arg(short, long)]
    pub help: bool,

    /// Print **Lua Definition File** `wefter.d.lua` directly to terminal
    #[arg(long)]
    pub meta: bool,

    /// Enable Info(**v**), Debug(**vv**) or Trace(**vvv**) messages
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Override project root *(Current working directory by default)*
    #[arg(long)]
    pub root: Option<PathBuf>,

    /// Folder to lookup for local profiles
    #[arg(short = 'd', long, default_value = ".wefter")]
    pub local_resources: PathBuf,

    /// List available profiles *(Profiles are read from resource directories)*
    #[arg(long)]
    pub profiles: bool,

    /// List command structure for the current profile
    #[arg(long, short = 'l')]
    pub commands: bool,

    /// Profile to use, if not present use **'auto.lua'**
    #[arg(short, long)]
    pub profile: Option<String>,

    /// Trailing parameters, this ones are sent directly to the profile configuration
    pub trailing: Vec<String>,
}
