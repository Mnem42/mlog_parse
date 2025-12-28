use std::{error::Error, fmt};

/// An error when parsing a statement
#[derive(Debug)]
pub enum StatementParseError<'s> {
    /// Missing jump label
    MissingJumpLabel(&'s str),
    /// Invalid instruction
    InvalidInstruction(Vec<&'s str>),
}

impl fmt::Display for StatementParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MissingJumpLabel(x) => format!("The jump label {x} is missing"),
                Self::InvalidInstruction(x) =>
                    format!("The instruction \"{}\" is invalid", x.join(",")),
            }
        )
    }
}

impl Error for StatementParseError<'_> {}
