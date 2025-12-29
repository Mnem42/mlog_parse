use regex::RegexSet;
use std::fmt::{self, Display, Write};
use std::num::{IntErrorKind, ParseIntError};
use std::str::FromStr;
use std::sync::LazyLock;
use strum::EnumString;

use super::instr_gen::gen_instructions;
use super::parse_nradix_literal;

/// An argument
#[derive(Debug, PartialEq)]
pub enum Argument<'a> {
    /// A numeric argument
    Number(f64),
    /// A literal string
    String(&'a str),
    /// A variable usage
    Variable(&'a str),
    /// A colour
    Colour(Rgba),
    // _^ British spotted
    /// A string starting in @
    GlobalConst(&'a str),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// A color
pub struct Rgba {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8,
}

impl Default for Rgba {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { r, g, b, a } = self;
        f.write_char('%')?;
        for color in [r, g, b, a] {
            write!(f, "{color:0>2x}")?;
        }
        Ok(())
    }
}

impl FromStr for Rgba {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut channels = (0..s.len()).step_by(2).map(|start| s.get(start..start + 2));
        let mut get_channel = || {
            let Some(Some(channel)) = channels.next() else {
                return u8::from_str_radix("", 16);
            };
            u8::from_str_radix(channel, 16)
        };
        let mut color = Self::default();
        let Self { r, g, b, a } = &mut color;
        for channel in [r, g, b] {
            *channel = get_channel()?;
        }
        match get_channel() {
            Err(err) if *err.kind() == IntErrorKind::Empty => {}
            Err(err) => return Err(err),
            Ok(alpha) => *a = alpha,
        }
        Ok(color)
    }
}

/// A conditional. [reference](https://github.com/Anuken/Mindustry/blob/master/core/src/mindustry/logic/ConditionOp.java)
#[derive(Debug, PartialEq, Eq, EnumString)]
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

impl fmt::Display for Argument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{x}"),
            Self::String(x) => write!(f, "\"{x}\""),
            Self::Variable(x) => write!(f, "{x}"),
            Self::Colour(x) => write!(f, "%{x}"),
            Self::GlobalConst(x) => write!(f, "@{x}"),
        }
    }
}

static ARG_PATTERNS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        "^\".*\"$",
        "^[%@][0-9a-fA-F]{6}(?:[0-9a-fA-F]{2})?$",
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
            _ if matches.matched(1) => Argument::Colour(value[1..].parse().unwrap()),
            _ if matches.matched(2) => Argument::GlobalConst(&value[1..]),
            _ if matches.matched(3) => {
                let first = value.as_bytes().first().copied().unwrap_or_default();
                let value = matches!(first, b'-' | b'+')
                    .then(|| &value[1..])
                    .unwrap_or(value);
                Argument::Number(
                    value.parse::<f64>().unwrap() * if first == b'-' { -1. } else { 1. },
                )
            }
            _ if matches.matched(4) => Argument::Number(parse_nradix_literal(value, 16) as f64),
            _ if matches.matched(5) => Argument::Number(parse_nradix_literal(value, 2) as f64),
            _ => Argument::Variable(value),
        }
    }
}

gen_instructions! {
    Statement,
    0i0o:
        Noop("nop")  = "No-op"
        Stop("stop") = "Stop execution"
        End("end")   = "End execution"

        UCIdle("ucontrol" "unbind")   = "Unit idle"
        UCStop("ucontrol" "stop")     = "Unit stop"
        UCUnbind("ucontrol" "unbind") = "Unit unbind"

        UCAutoPathfind("ucontrol" "autoPathFind") = "Unit auto pathfind"
        UCPayloadDrop("ucontrol" "payDrop")       = "Unit drop payload"
        UCPayloadEnter("ucontrol" "payEnter")     = "Unit enter payload containing block"

        DrawReset("draw" "reset") = ""
    ---

    1i0o:
        DrawCol("draw" "col")       = "Set draw colour"
        DrawStroke("draw" "stroke") = "Set draw stroke size"

        Print("print")         = "Print a string"
        PrintChar("printchar") = "Print a char based on its ASCII code"
        Format("format")       = "Print a string with format substitutions"

        DrawFlush("drawflush")   = "Draw the drawbuffer to a display"
        PrintFlush("printflush") = "Print the messagebuffer to a message block"

        UBind("ubind") = "Bind a unit with a given type"

        UCBoost("ucontrol" "boost")   = "Set whether a unit should boost"
        UCPayTake("ucontrol" "payTake") = "Make a unit take payload"
        UCFlag("ucontrol" "flag")     = "Sets a unit's flag"

        Wait("wait") = "Wait for n seconds"
    ---

    2i0o:
        DrawTranslate("draw" "translate") = "Translate everything in the print buffer"
        DrawRotate("draw" "rotate")       = "Rotate everything in the print buffer"
        DrawScale("draw" "scale")         = "Scale everything in the print buffer"

        ControlEnabled("control" "enabled") = "Set whether a block is enabled or not"
        ControlConfig("control" "config")   = "Set the configuration of a block (exact behaviour depends on the block)"
        ControlColour("control" "color")    = "Set the colour of a block that supports it"

        UCMove("ucontrol" "move")         = "Set the position for units to move to"
        UCPathfind("ucontrol" "pathfind") = "Set the position for units to pathfind to"
        UCTargetP("ucontrol" "targetp")   = "Makes a unit shoot with velocity prediction"
        UCItemDrop("ucontrol" "itemDrop") = "Makes a unit drop items"
        UCMine("ucontrol" "mine")         = "Makes a unit mine a given coordinate"
    ---


    3i0o:
        ControlShootP("control shootp") = "Set where a turret should shoot with velocity prediction"

        UCApproach("ucontrol" "approach") = "Set the position for units to approach"
        UCTarget("ucontrol" "target")     = "Set the position for units to target"
        UCItemTake("ucontrol" "itemtake") = "Make a unit take items"
    ---

    4i0o:
        ControlShoot("control shoot") = "Set where a turret should shoot"
    ---

    1i1o:
        Set("set") = "Set variable"

        // Looks wrong, but isn't
        BlockLookup("lookup" "block")   = "Lookup a block by index"
        Unitookup("lookup" "unit")      = "Lookup a unit by index"
        ItemLookup("lookup" "item")     = "Lookup an item by index"
        LiquidLookup("lookup" "liquid") = "Lookup a liquid by index"
        TeamLookup("lookup" "team")     = "Lookup a team by index"

        GetLink("getlink") = "Get a link by index"
    ---

    2i1o:
        OpAdd("op" "add") = "Addition"
        OpSub("op" "sub") = "Subtraction"
        OpMul("op" "mul") = "Multiplication"
        OpDiv("op" "div") = "Division"
        OpExp("op" "pow") = "Exponentiation"

        OpIntDiv("op" "fdiv")  = "Integer division"
        OpMod("op" "mod")      = "Modulo"
        OpTrueMod("op" "emod") = "True modulo (gets sign from divisor)"

        OpEq("op" "equal")     = "Equality check"
        OpNot("op" "notEqual") = "Inequality check"

        OpRead("read")   = "Read from memory cell"
        OpWrite("write") = "Write to memory cell"
    ---

    4i1o:
        PackColour("packcolor") = "Pack a colour from RGBA values"
    ---
}
