//! Error types for recursive copy operations.
//!
//! This module defines the `CopyError` enum, which categorizes various
//! failures that can occur during the file copying process, such as
//! IO issues, depth limits, or symlink recursion.

use std::fmt;
use std::{io, path::PathBuf};
use walkdir_minimal::WalkError;

/// Enumeration of possible errors during a copy operation.
#[derive(Debug)]
pub enum CopyError {
    /// Standard input/output error.
    Io(io::Error),
    /// Error encountered while traversing the directory tree.
    Walk(WalkError),
    /// The directory traversal reached the maximum configured depth.
    DepthExceeded(PathBuf),
    /// A circular reference was detected through symbolic links.
    SymlinkLoop(PathBuf),
    /// The specified source path does not exist.
    SrcNotFound(PathBuf),
    /// The destination exists but is not a directory when one was expected.
    DestNotDir(PathBuf),
    /// Encountered a file type or operation that is not supported (e.g., special devices).
    NotSupported(PathBuf),
}

impl fmt::Display for CopyError {
    /// Formats the error for user-facing display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CopyError::Io(e) => write!(f, "IO error: {}", e),
            CopyError::Walk(e) => write!(f, "Walk error: {:?}", e),
            CopyError::DepthExceeded(p) => write!(f, "Maximum depth exceeded at: {}", p.display()),
            CopyError::SymlinkLoop(p) => write!(f, "Symlink loop detected: {}", p.display()),
            CopyError::SrcNotFound(p) => write!(f, "Source not found: {}", p.display()),
            CopyError::DestNotDir(p) => write!(f, "Destination is not a directory: {}", p.display()),
            CopyError::NotSupported(p) => write!(f, "Operation not supported at: {}", p.display()),
        }
    }
}

impl std::error::Error for CopyError {
    /// Returns the underlying cause of the error if it exists.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CopyError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for CopyError {
    /// Automatically converts `std::io::Error` into `CopyError::Io`.
    fn from(e: io::Error) -> Self {
        CopyError::Io(e)
    }
}

impl From<WalkError> for CopyError {
    /// Automatically converts `WalkError` into `CopyError::Walk`.
    fn from(e: WalkError) -> Self {
        CopyError::Walk(e)
    }
}
