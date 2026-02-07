use std::path::PathBuf;
use thiserror::Error;

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
}
