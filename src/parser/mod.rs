/// Arguments
pub mod args;
/// Error handling
pub mod errs;
/// The definition of the Statement type.
pub mod statements;

/// The lexer.
pub mod lexer;

pub use statements::Statement;
pub use lexer::Lexer;
pub use errs::ParseError;
pub use statements::ParseError as StatementParseErr;