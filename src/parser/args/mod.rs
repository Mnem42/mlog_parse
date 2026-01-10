//! This module defines all things that can be passed in as arguments to a statement like colours,
//! strings, and numbers.

/// Provides the [`Rgba`] type for colours.
pub mod colour;
/// Definitions for mindustry's named colours (e.g. `red`, `blue`)
pub mod named_colours;

mod num_parse;

#[cfg(test)]
mod test;

use num_parse::parse_nradix_literal;
use regex::RegexSet;
use std::fmt;
use std::sync::LazyLock;
use strum::EnumString;

pub use colour::Rgba;

/// An argument that can be passed into a statement
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Argument<'a> {
    /// A numeric argument (e.g. `5`, `0x2763`, `0b0011s`)
    Number(f64),
    /// A literal string (e.g. `"hello world"`)
    String(&'a str),
    /// A variable usage. This is used for anything that doesn't match any of the other types,
    /// which is also how the game handles it.
    Variable(&'a str),
    /// A colour literal (e.g. `%01234567`, `%deadbeef`)
    Colour(Rgba),
    // _^ British spotted
    /// A global constant (e.g. `@counter`, `@thisx`, `@thisy`)
    GlobalVar(&'a str),
}

impl fmt::Display for Argument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{x}"),
            Self::String(x) => write!(f, "\"{x}\""),
            Self::Variable(x) => write!(f, "{x}"),
            Self::Colour(x) => write!(f, "%{x}"),
            Self::GlobalVar(x) => write!(f, "@{x}"),
        }
    }
}

static ARG_PATTERNS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        "^\".*\"$",
        "^%[0-9a-fA-F]{6}(?:[0-9a-fA-F]{2})?$",
        "^%[.*]",
        "^@",
        r"^[+-]?\d+(?:\.\d+)?$",
        "^[+-]?0x[0-9a-fA-F]+$",
        "^[+-]?0b[01]+$",
    ])
    .unwrap()
});

impl<'s> From<&'s str> for Argument<'s> {
    fn from(value: &'s str) -> Self {
        let matches = ARG_PATTERNS.matches(value);
        match () {
            _ if matches.matched(0) => Argument::String(&value[1..value.len() - 1]),
            _ if matches.matched(1) => Argument::Colour(Rgba::from_hex_literal_unchecked(value)),
            _ if matches.matched(2) => {
                if let Some(colour) = Rgba::from_named_literal_unchecked(value) {
                    Argument::Colour(colour)
                } else {
                    Argument::Variable(value)
                }
            }
            _ if matches.matched(3) => Argument::GlobalVar(&value[1..]),
            _ if matches.matched(4) => {
                let first = value.as_bytes().first().copied().unwrap_or_default();
                let value = matches!(first, b'-' | b'+')
                    .then(|| &value[1..])
                    .unwrap_or(value);
                Argument::Number(
                    value.parse::<f64>().unwrap() * if first == b'-' { -1. } else { 1. },
                )
            }
            _ if matches.matched(5) => Argument::Number(parse_nradix_literal(value, 16) as f64),
            _ if matches.matched(6) => Argument::Number(parse_nradix_literal(value, 2) as f64),
            _ => Argument::Variable(value),
        }
    }
}

/// A conditional. [reference](https://github.com/Anuken/Mindustry/blob/master/core/src/mindustry/logic/ConditionOp.java).
/// This is used for the `select` and `jump` instructions.
#[derive(Debug, PartialEq, Eq, EnumString, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConditionOp {
    /// Equality (==)
    #[strum(serialize = "equal")]
    Equal,
    /// Inequality (!=)
    #[strum(serialize = "notEqual")]
    NotEqual,
    /// Strict inequality (!=)
    #[strum(serialize = "strictNotEqual")]
    StrictNotEqual,
    /// Less than (<)
    #[strum(serialize = "lessThan")]
    LessThan,
    /// Less than or equal to (<=)
    #[strum(serialize = "lessThanEq")]
    LessThanEq,
    /// Greater than (>)
    #[strum(serialize = "greaterThan")]
    GreaterThan,
    /// Greater than or equal to (>=)
    #[strum(serialize = "greaterThanEq")]
    GreaterThanEq,
    /// Strict equality (i.e. no type coercion) (===)
    #[strum(serialize = "strictEqual")]
    StrictEqual,
    /// Always jump, with no condition
    #[strum(serialize = "always")]
    Always,
}

impl fmt::Display for ConditionOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equal => "equal",
                Self::NotEqual => "notEqual",
                Self::StrictNotEqual => "strictNotEqual",
                Self::LessThan => "lessThan",
                Self::LessThanEq => "lessThanEq",
                Self::GreaterThan => "greaterThan",
                Self::GreaterThanEq => "greaterThanEq",
                Self::StrictEqual => "strictEqual",
                Self::Always => "always",
            }
        )
    }
}
