use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoomErr {
    #[error("Cannot find system directories ($HOME/.config and $HOME/.local/share)")]
    FilesystemError,

    #[error("Specified resource directory does not exist: {0:?}")]
    NoSuchResourceDirectory(PathBuf),

    #[error("Found an invalid resource at: {0:?}")]
    InvalidResource(PathBuf),

    #[error("Unknown kind {0}, no matching resources found")]
    UnknownKind(String),
}
