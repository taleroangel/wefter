use std::{cell::RefCell, path::PathBuf, rc::Rc};

/// Variant to tell which action was executed
pub enum HistoryAction {
    /// Path to the file created
    CreateFile(PathBuf),

    /// Path to the file modified and insertion point string
    ModifyFile(PathBuf, String),
}

/// Ordered list of actions on the file system
pub type History = Vec<HistoryAction>;

/// Mutable reference 
pub type HistoryRef = Rc<RefCell<History>>;
