use anyhow::{Ok, Result};
use fern::colors::{Color, ColoredLevelConfig};

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
