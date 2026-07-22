use std::{cell::RefCell, path::PathBuf, rc::Rc};

/// Variant to tell which action was executed
pub enum HistoryAction {
    /// Path to the file created
    CreateFile(PathBuf),

    /// Path to the file modified and insertion point string
    ModifyFile(PathBuf, String),

    /// Path to the directory created
    CreateDirectory(PathBuf),

    /// [previous] file was renamed and now is [new]
    FileRenamed { previous: PathBuf, new: PathBuf },

    /// [previous] file was moved and now lives at [new]
    FileMoved{ previous: PathBuf, new: PathBuf },
}

/// Ordered list of actions on the file system
pub type History = Vec<HistoryAction>;

/// Mutable reference 
pub type HistoryRef = Rc<RefCell<History>>;
