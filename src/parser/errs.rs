use std::{error::Error, fmt};

/// An error when parsing a statement
#[derive(Debug)]
pub enum StatementParseError {
    /// Missing jump label
    MissingJumpLabel(String),
    /// Invalid instruction
    InvalidInstruction(Vec<String>)
}

impl fmt::Display for StatementParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MissingJumpLabel(x) => format!("The jump label {} is missing", x),
                Self::InvalidInstruction(x) => format!("The instruction \"{}\" is invalid", x.join(","))
            }
        )
    }
}

impl Error for StatementParseError {}
