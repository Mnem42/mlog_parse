use std::{error::Error, fmt};

#[derive(Debug)]
pub enum InstructionParseError {
    MissingJumpLabel(String)
}

impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::MissingJumpLabel(x) => format!("The jump label {} is missing", x)
        })
    }
}

impl Error for InstructionParseError {}