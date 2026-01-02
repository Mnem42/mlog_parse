/// Instructions
pub mod args;
/// Error handling
pub mod errs;
/// The definition of the Statement type.
pub mod statements;

pub mod lexer;

use errs::StatementParseError;

use crate::parser::statements::StatementType;
