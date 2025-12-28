use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use strum::EnumString;

use super::instr_gen::gen_instructions;
use super::parse_nradix_literal;

/// An argument
#[derive(Debug, PartialEq)]
pub enum Argument {
    /// A numeric argument
    Number(f64),
    /// A literal string
    String(String),
    /// A variable usage
    Variable(String),
    /// A colour
    Colour(String)
}

/// A conditional. [reference](https://github.com/Anuken/Mindustry/blob/master/core/src/mindustry/logic/ConditionOp.java)
#[derive(Debug, PartialEq, EnumString)]
pub enum ConditionOp {
    /// Equality (==)
    #[strum(serialize = "equal")]
    Equal,
    /// Inequality (!=)
    #[strum(serialize = "notEqual")]
    NotEqual,
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

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{}", x),
            Self::String(x) => write!(f, "\"{}\"", x),
            Self::Variable(x) => write!(f, "{}", x),
            Self::Colour(x) => write!(f, "%{}", x)
        }
    }
}

lazy_static! {
    static ref HEX_REGEX: regex::Regex = Regex::new("^[+-]?0x[0-9a-fA-F]+$").unwrap();
    static ref BIN_REGEX: regex::Regex = Regex::new("^[+-]?0b[01]+$").unwrap();
    static ref COLOUR_REGEX: regex::Regex = Regex::new("^%[09a-fA-F]{6}(?:[09a-fA-F]{2})?$").unwrap();
}

impl From<&str> for Argument {
    fn from(value: &str) -> Self {
        if let Ok(x) = value.parse() {
            Argument::Number(x)
        } else if value.starts_with('"') && value.ends_with('"') {
            Argument::String(value[1..value.len() - 1].to_string())
        } else if COLOUR_REGEX.is_match(value){
            Argument::Colour(value.strip_prefix("%").unwrap().to_string())
        } else if HEX_REGEX.is_match(value) {
            Argument::Number(parse_nradix_literal(value, 16) as f64)
        } else if BIN_REGEX.is_match(value) {
            Argument::Number(parse_nradix_literal(value, 2) as f64)
        } else {
            Argument::Variable(value.to_string())
        }
    }
}

gen_instructions! {
    Statement,
    0i0o:
        Noop("nop") = "No-op"

        UCIdle("ucontrol" "unbind") = "Unit idle"
        UCStop("ucontrol" "stop") = "Unit stop"
        UCAutoPathfind("ucontrol" "autoPathFind") = "Unit auto pathfind"
        UCPayloadDrop("ucontrol" "payDrop") = "Unit drop payload"
        UCPayloadEnter("ucontrol" "payEnter") = "Unit enter payload containing block"
        UCUnbind("ucontrol" "unbind") = "Unit unbind"

        DrawReset("draw" "reset") = ""
    ---

    1i0o:
        DrawCol("draw" "col") = "Set draw colour"
        DrawStroke("draw" "stroke") = "Set draw stroke size"
        DrawFlush("drawflush") = "Flush draw buffer to provided display"
    ---

    2i0o: 
        DrawTranslate("draw" "translate") = "Translate everything in the print buffer"
        DrawRotate("draw" "rotate") = "Rotate everything in the print buffer"
        DrawScale("draw" "scale") = "Scale everything in the print buffer"
    ---

    1i1o:
        Set("set") = "Set variable"
    ---

    2i1o:
        OpAdd("op" "add") = "Addition"
        OpSub("op" "sub") = "Subtraction"
        OpMul("op" "mul") = "Multiplication"
        OpDiv("op" "div") = "Division"
        OpExp("op" "pow") = "Exponentiation"

        OpIntDiv("op" "fdiv") = "Integer division"
        OpMod("op" "mod") = "Modulo"
        OpTrueMod("op" "emod") = "True modulo (gets sign from divisor)"

        OpEq("op" "equal") = "Equality check"
        OpNot("op" "notEqual") = "Inequality check"

        OpRead("read") = "Read from memory cell"
        OpWrite("write") = "Write to memory cell"
    ---
}