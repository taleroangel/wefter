use std::path::PathBuf;
use clap::Parser;

/// Input parameters for the program
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Params {
    /// Verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Override project root
    #[arg(long)]
    pub root: Option<String>,

    /// Override configuration file
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Kind of project, if not present, use 'autodetect'
    #[arg(short, long)]
    pub kind: Option<String>,
}
