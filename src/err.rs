//! Application errors, and related behaviors.

use thiserror::Error;

/// Top-level application error.
#[derive(Debug, Error)]
pub enum SysErr {
    /// Error processing input or output from the system.
    #[error(transparent)]
    IOErr(#[from] IOErr),
}

/// Error processing input or output from the system.
#[derive(Debug, Error)]
#[error("error processing input or output from the system")]
pub struct IOErr {
    /// Source error.
    #[from]
    pub src: std::io::Error,
}
