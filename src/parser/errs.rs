//! This module implements error handling for the parser.

use std::{error::Error, fmt};
use crate::parser::statements;

/// An error found when parsing
#[derive(Debug, PartialEq)]
pub enum ParseError<'s> {
    /// Any invalid statement
    Statement {
        /// The line it occurred on (in the source)
        line: usize,
        /// The specific error that occurred
        error: statements::ParseError<'s>
    }
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Statement { line, error } => {
                    match error {
                        statements::ParseError::MissingJumpLabel(x) => 
                            format!("The jump label {x} is missing (used on line {line})"),
                        statements::ParseError::InvalidInstruction(x) =>
                            format!("The instruction \"{}\" is invalid  (line {line})", x.join(",")),
                    }
                }
            }
        )
    }
}

impl Error for ParseError<'_> {}
