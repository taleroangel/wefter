use std::path::PathBuf;
use thiserror::Error;

/// Loom-specific errors
#[derive(Debug, Error)]
pub enum LoomErr {
    #[error("Cannot find system directories ($HOME/.config and $HOME/.local/share)")]
    FilesystemError,

    #[error("Cannot use {0:?} as the root directory")]
    BadRootDirectory(PathBuf),

    #[error("Specified resource directory does not exist: {0:?}")]
    NoSuchResourceDirectory(PathBuf),

    #[error("Found an invalid resource at: {0:?}")]
    InvalidResource(PathBuf),

    #[error("Unknown profile {0}, no matching resources found")]
    UnknownProfile(String),

    #[error("No available profiles")]
    NoAvailableProfiles,

    #[error("Lua file {0:?} does not exist")]
    NoSuchLuaFile(PathBuf),

    #[error("No profile was specified and auto returned no coincidences")]
    NoProfileSpecified,

    #[error("Failed to parse profile definition, bad structure")]
    BadProfileDefinition,

    #[error("In (lua) file {0:?}")]
    BadLuaExec(PathBuf, #[source] mlua::Error),

    #[error("Expected one of the following commands {0:?}")]
    EmptyParameters(Vec<String>),

    #[error("Bad profile parameters. Command `{0}` is not defined")]
    CommandNotFound(String),

    #[error("Command `{0}` expected one of the following subcommands {1:?}")]
    MissingSubcommand(String, Vec<String>),

    #[error("No such subcommand `{subcommand}`, for command `{command}`")]
    SubcommandNotFound { command: String, subcommand: String },
}
