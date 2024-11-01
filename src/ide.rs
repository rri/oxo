//! Integrated Development Environment (IDE), which constitutes a shell and associated behaviors.

use crate::err::IOErr;

/// Shell for interactive program development.
#[derive(Clone, Debug)]
pub struct Shell;

impl Shell {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }

    /// Run the interactive shell until it crashes or the user quits.
    pub fn run(&self) -> Result<(), IOErr> {
        Ok(())
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}
