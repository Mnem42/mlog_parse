//! A parser for mindustry logic.
//!
//! **NOTE:** All minor version updates are breaking changes for now (the API isn't stabilised)
//!
//! # Examples
//!
//! ```
//! # fn main() {
//! # use mlog_parse::parser::lexer::Lexer;
//! # use mlog_parse::parser::statements::Statement;
//! # use mlog_parse::parser::args::Argument;
//! # use mlog_parse::parser::args::ConditionOp;
//! const SRC: &str = r#"
//!     loop_start:
//!         op add i i 1
//!         write i cell1 0
//!     jump loop_start lessThan i 5
//! "#;
//!
//! let lexer: Lexer<Statement> = Lexer::new(SRC);
//!
//! let instructions: Vec<_> = lexer
//!     .map(|x| x.unwrap())
//!     .collect();
//!
//! assert_eq!(
//!     instructions,
//!     vec![
//!         Statement::OpAdd{
//!             a: Argument::Variable("i"),
//!             b: Argument::Number(1.),
//!             c: "i"
//!         },
//!         Statement::Write {
//!             value: Argument::Variable("i"),
//!             cell:  Argument::Variable("cell1"),
//!             index: Argument::Number(0.)
//!         },
//!         Statement::Jump {
//!            index: 0,
//!            cond: ConditionOp::LessThan,
//!            lhs: Some(Argument::Variable("i")),
//!            rhs: Some(Argument::Number(5.)),
//!         }
//!     ]
//! );
//! # }
//! ```

#![warn(missing_docs)]

/// The module for parsing
pub mod parser;

#[cfg(test)]
mod tests;
