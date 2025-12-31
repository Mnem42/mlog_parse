macro_rules! gen_match_l {
    (
        oi
        $($name:literal),* 
        $($i:ident),* -> $($o:ident),*
    ) => { [$($name),*, $($i,)* $($o,)* ..] };
    (
        io
        $($name:literal),* 
        $($i:ident),* -> $($o:ident),*
    ) => { [$($name),*, $($i,)* $($o,)* ..] };
}
macro_rules! gen_match_guard {
    ($($o:ident)*) => { $(matches!(Argument::from(*$o), Argument::Variable(_))&&)* true };
}
macro_rules! gen_match_result {
    (
        $enum:ident
        $ident:ident
        $($i:ident),* -> $($o:ident),*
    ) => {
        $enum::$ident {
            $($o,)*
            $($i: Argument::from(*$i)),*
        }
    }
}

macro_rules! gen_enum {
    (
        $enum: ident
        $($ident:ident $($i:ident),* -> $($o:ident),*);*
    ) => {
        /// A statement
        #[derive(Debug, PartialEq)]
        pub enum $enum<'a> {
            Jump {
                /// The index to jump to
                index: usize,
                /// The condition
                cond: ConditionOp,
                /// The condition LHS
                lhs: Option<Argument<'a>>,
                /// The condition RHS
                rhs: Option<Argument<'a>>
            },
            $($ident {
                $($i: Argument<'a>,)*
                $($o: &'a str),*
            }),*,
        }
    };
}

macro_rules! gen_printer {
    (oi $f:expr ; $($name:literal),* $($i:ident),* -> $($o:ident),*) => {
        (|| {
            $f.write_str(concat!("" $(, $name ,)" "*))?;
            $(write!($f, " {}", $o)?;)*
            $(write!($f, " {}", $i)?;)*
            Ok(())
        })()
    };
    (io $f:expr ; $($name:literal),* $($i:ident),* -> $($o:ident),*) => {{
        (|| {
            $f.write_str(concat!("" $(, $name ,)" "*))?;
            $(write!($f, " {}", $i)?;)*
            $(write!($f, " {}", $o)?;)*
            Ok(())
        })()
    }};
}

/// Generates a statements enum
/// 
/// `oi` means the outputs are *before* the inputs in the statement, while `io` means the inputs
/// are *before* the outputs in the statement.
macro_rules! gen_statements {
    {
        $enum: ident, 
        $(
            $ident:ident:
            $($name:literal)*
            ($ty:tt: $($i:ident),* -> $($o:ident),*)
        )*
    } => {mod thing {
        use crate::parser::instructions::Argument;
        use crate::parser::errs::StatementParseError;
        use crate::parser::instructions::ConditionOp;

        gen_enum!{
            $enum
            $($ident $($i),* -> $($o),*);*
        }

        impl<'a> $enum<'a> {
            /// Parses a token
            pub fn parse(
                tokens: &[&'a str], 
                jump_labels: &std::collections::HashMap<&'a str, usize>
            ) -> Result<Self, StatementParseError<'a>> {
                match tokens {
                    ["jump", index, cond_str, lhs, rhs, ..] if ConditionOp::try_from(*cond_str).is_ok() => {
                        if let Ok(index) = index.parse() {
                            Ok(Self::Jump {
                                index,
                                cond: ConditionOp::try_from(*cond_str).unwrap(),
                                lhs: Some(Argument::from(*lhs)),
                                rhs: Some(Argument::from(*rhs))
                            })
                        }
                        else {
                            Ok(Self::Jump {
                                index: jump_labels
                                    .get(*index)
                                    .ok_or(StatementParseError::MissingJumpLabel(index))?
                                    .clone(),
                                cond: ConditionOp::try_from(*cond_str).unwrap(),
                                lhs: Some(Argument::from(*lhs)),
                                rhs: Some(Argument::from(*rhs))
                            })
                        }
                    },
                    ["jump", index, "always", ..] => {
                        if let Ok(index) = index.parse() {
                            Ok(Self::Jump {
                                index,
                                cond: ConditionOp::Always,
                                lhs: None,
                                rhs: None
                            })
                        }
                        else {
                            Ok(Self::Jump {
                                index: jump_labels
                                    .get(*index)
                                    .ok_or(StatementParseError::MissingJumpLabel(index))?
                                    .clone(),
                                cond: ConditionOp::Always,
                                lhs: None,
                                rhs: None
                            })
                        }
                    },
                    $(
                        gen_match_l!($ty $($name),* $($o),* -> $($i),*)
                            if gen_match_guard!($($o)*)
                        => Ok(gen_match_result!($enum $ident $($i),* -> $($o),*)),
                    )*
                    _ => Err(StatementParseError::InvalidInstruction(tokens.to_vec()))
                }
            }
        }

        impl std::fmt::Display for $enum<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    Self::Jump { index, cond, lhs: None, rhs: None } => 
                        write!(f, "jump {} {}", index, cond),
                    Self::Jump { index, cond, lhs: Some(lhs), rhs: Some(rhs) } =>
                        write!(f, "jump {} {} {} {}", index, cond, lhs, rhs),
                    $(
                        Self::$ident {$($i),* $(,$o)*} => {
                            gen_printer!($ty f ; $($name),* $($i),* -> $($o),*)
                        },
                    )*
                    _ => unreachable!()
                }
            }
        }
    }
}}


gen_statements!{
    Statement,

    Noop: "nop"  (io: ->)
    Stop: "stop" (io: ->)
    End:  "end"  (io: ->)
    Set:  "set"  (oi: value -> var)
    Wait: "wait" (io: time ->)

    GetLink: "getlink" (oi: index -> result)
    
    Sensor: "sensor"   (oi: item -> result)

    Read:  "read"  (oi: cell, index -> result)
    Write: "write" (io: value, cell, index ->)

    Print:      "print"      (io: text ->)
    PrintChar:  "printChar"  (io: char ->)
    Format:     "format"     (io: f_string ->)
    PrintFlush: "printflush" (io: output ->)

    PackColour: "packcolor"     (oi: r, g, b, a -> result)
    DrawReset:  "draw" "reset"  (io: ->)
    DrawCol:    "draw" "col"    (io: packed_colour ->)
    DrawStroke: "draw" "stroke" (io: width ->)
    DrawFlush:  "drawflush"     (io: output ->)
    DrawTranslate: "draw" "translate" (io: x, y ->)
    DrawRotate:    "draw" "rotate"    (io: angle ->)
    DrawScale:     "draw" "scale"     (io: x, y ->)

    ControlEnabled: "control" "enabled" (io: enabled ->)
    ControlConfig:  "control" "config"  (io: config ->)
    ControlColour:  "control" "color"   (io: colour ->)
    ControlShoot:   "control" "shoot"   (io: block, x, y, shoot ->)
    ControlShootP:  "control" "shootp"  (io: block, unit, shoot ->)

    BlockLookup:  "lookup" "block"  (oi: index -> result)
    UnitLookup:   "lookup" "unit"   (oi: index -> result)
    ItemLookup:   "lookup" "item"   (oi: index -> result)
    LiquidLookup: "lookup" "liquid" (oi: index -> result)
    TeamLookup:   "lookup" "team"   (oi: index -> result)

    OpAdd: "op" "add" (oi: a, b -> c)
    OpSub: "op" "sub" (oi: a, b -> c)
    OpMul: "op" "mul" (oi: a, b -> c)
    OpDiv: "op" "div" (oi: a, b -> c)
    OpExp: "op" "pow" (oi: a, b -> c)
    OpIntDiv:   "op" "fdiv" (oi: a, b -> c)
    OpMod:      "op" "mod"  (oi: a, b -> c)
    OpTrueMod:  "op" "emod" (oi: a, b -> c)
    OpEq:       "op" "equal"    (oi: a, b -> result)
    OpNotEqual: "op" "notEqual" (oi: a, b -> result)

    UBind:   "ubind"   (io: unit_type ->)
    ULocate: "ulocate" (io: find, group, enemy, outx, outy -> found, building)

    UCIdle:     "ucontrol" "idle"   (oi: ->)
    UCStop:     "ucontrol" "stop"   (oi: ->)
    UCUnbind:   "ucontrol" "unbind" (oi: ->)
    UCFlag:     "ucontrol" "flag"   (oi: flag ->)
    UCMove:     "ucontrol" "move"     (oi: x, y ->)
    UCPathfind: "ucontrol" "pathfind" (oi: x, y ->)
    UCApproach: "ucontrol" "approach" (oi: x, y, radius ->)
    UCBoost:    "ucontrol" "boost"    (oi: boost ->)
    UCMine:     "ucontrol" "move"     (oi: x, y ->)
    UCTarget:   "ucontrol" "target"   (oi: x, y, shoot ->)
    UCTargetP:  "ucontrol" "move"     (oi: unit, shoot ->)
    UCPayTake:  "ucontrol" "payTake"  (oi: units ->)
    UCPayDrop:  "ucontrol" "payDrop"  (oi: ->)
    UCItemTake: "ucontrol" "itemTake" (oi: x, y, radius ->)
    UCItemDrop: "ucontrol" "itemDrop" (oi: to, amount ->)
    UCAutoPathfind: "ucontrol" "autoPathFind" (oi: ->)
    UCPayloadDrop:  "ucontrol" "payDrop"      (oi: ->)
    UCPayloadEnter: "ucontrol" "payEnter"     (oi: ->)
}

use std::process::Output;

pub use thing::Statement;