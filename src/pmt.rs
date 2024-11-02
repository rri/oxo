//! Shell prompts and associated behaviors.

use crate::fmt::Formatted;
use std::fmt::Display;
use std::fmt::Formatter;

/// Displayable prompt.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Prompt {
    /// Prompt for user input.
    Inp,
    /// Prompt for continuation of user input.
    Cnt,
    /// Prompt for system output displayed to the user.
    Out,
    /// Prompt for system errors displayed to the user.
    Err,
}

const INP: &str = "→";
const CNT: &str = "↓";
const OUT: &str = "≡";
const ERR: &str = "✕";

impl Display for Prompt {
    fn fmt(&self, f: &mut Formatter) -> Formatted {
        match self {
            Self::Inp => write!(f, "{} ", INP),
            Self::Cnt => write!(f, "{} ", CNT),
            Self::Out => write!(f, "{} ", OUT),
            Self::Err => write!(f, "{} ", ERR),
        }
    }
}
