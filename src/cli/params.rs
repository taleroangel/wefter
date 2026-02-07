use clap::Parser;
use std::path::PathBuf;

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

    /// Folder to lookup (project) local configs & templates
    #[arg(short = 'd', long, default_value = ".loom")]
    pub local_res_dir: PathBuf,

    /// Kind of project, if not present use 'autodetect'
    #[arg(short, long)]
    pub kind: Option<String>,
}
