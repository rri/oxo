//! Core application library.

use crate::err::SysErr;
use crate::ide::Shell;
use clap::Parser;

/// Core application.
#[derive(Clone, Debug, Parser)]
#[command(about = None, long_about = None, version, arg_required_else_help(true))]
pub struct App {
    /// Launch in interactive development mode.
    #[arg(short, long)]
    pub interactive: bool,
}

impl App {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::parse()
    }

    /// Run this application.
    pub fn run(&self) -> Result<(), SysErr> {
        if self.interactive {
            Shell::new().run()?;
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
