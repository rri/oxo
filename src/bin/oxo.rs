//! Main entry point for the application.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use anyhow::Result;
use oxo::app::App;

/// Main entry point function for the application.
pub fn main() -> Result<()> {
    App::new().run()?;
    Ok(())
}
