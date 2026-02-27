//! Configuration options for the copy process.
//!
//! This module contains the `CopyOptions` struct, which allows users to
//! customize how files, directories, and symlinks are handled during
//! the recursive copy operation.

/// Configuration settings for the recursive copy operation.
#[derive(Clone, Debug)]
pub struct CopyOptions {
    /// If true, existing files at the destination will be replaced.
    pub overwrite: bool,
    /// If true, prevents symlinks from pointing to locations outside the source tree.
    pub restrict_symlinks: bool,
    /// If true, copies the actual content of the symlink target instead of the link itself.
    pub follow_symlinks: bool,
    /// If true, copies only the contents of the source directory, not the directory itself.
    pub content_only: bool,
    /// The size of the buffer used for file I/O operations (in bytes).
    pub buffer_size: usize,
    /// The maximum recursion depth for directory traversal.
    pub depth: usize,
}

impl Default for CopyOptions {
    /// Provides default values for `CopyOptions`.
    fn default() -> Self {
        Self {
            overwrite: false,
            restrict_symlinks: false,
            follow_symlinks: false,
            content_only: false,
            buffer_size: 64 * 1024,
            depth: 512,
        }
    }
}
