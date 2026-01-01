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
/*macro_rules! gen_match_guard {
    ($($o:ident)*) => { $(matches!(Argument::from(*$o), Argument::Variable(_) | Argument::GlobalConst(_))&&)* true };
}*/
macro_rules! gen_match_result {
    (
        $enum:ident
        $ident:ident
        $($i:ident),* -> $($o:ident),*
    ) => {
        $enum::$ident {
            $($o: 
                if let Argument::Variable(x) | Argument::GlobalConst(x) = Argument::from(*$o) 
                    { Some(x) } else { None },
            )*
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
            Select {
                /// The index to jump to
                result: &'a str,
                /// The condition
                cond: ConditionOp,
                /// The condition LHS
                lhs: Option<Argument<'a>>,
                /// The condition RHS
                rhs: Option<Argument<'a>>,
                /// Option when true
                true_option: Argument<'a>,
                /// Option when false
                false_option: Argument<'a>
            },
            $($ident {
                $($i: Argument<'a>,)*
                $($o: Option<&'a str>),*
            }),*,
        }
    };
}

macro_rules! gen_printer {
    (oi $f:expr ; $($name:literal),* $($i:ident),* -> $($o:ident),*) => {
        (|| {
            $f.write_str(concat!("" $(, $name ,)" "*))?;
            $(write!($f, " {}", $o.unwrap_or("0"))?;)*
            $(write!($f, " {}", $i)?;)*
            Ok(())
        })()
    };
    (io $f:expr ; $($name:literal),* $($i:ident),* -> $($o:ident),*) => {{
        (|| {
            $f.write_str(concat!("" $(, $name ,)" "*))?;
            $(write!($f, " {}", $i)?;)*
            $(write!($f, " {}", $o.unwrap_or("0"))?;)*
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
                    ["select", result, cond_str, lhs, rhs, true_option, false_option, ..] if ConditionOp::try_from(*cond_str).is_ok() => {
                        Ok(Self::Select {
                            result,
                            cond: ConditionOp::try_from(*cond_str).unwrap(),
                            lhs: Some(Argument::from(*lhs)),
                            rhs: Some(Argument::from(*rhs)),
                            true_option: Argument::from(*true_option),
                            false_option: Argument::from(*false_option)
                        })
                    },
                    ["select", result, "always", true_option, false_option, ..] => {
                        Ok(Self::Select {
                            result,
                            cond: ConditionOp::Always,
                            lhs: None,
                            rhs: None,
                            true_option: Argument::from(*true_option),
                            false_option: Argument::from(*false_option)
                        })
                    },
                    $(
                        gen_match_l!($ty $($name),* $($o),* -> $($i),*)
                            /*if gen_match_guard!($($o)*)*/
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

gen_statements! {
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
    PrintChar:  "printchar"  (io: char ->)
    Format:     "format"     (io: f_string ->)
    PrintFlush: "printflush" (io: output ->)

    PackColour: "packcolor"     (oi: r, g, b, a -> result)
    DrawReset:  "draw" "reset"  (io: ->)
    DrawClear:  "draw" "clear"  (io: r, g, b ->)
    DrawCol:    "draw" "col"    (io: packed_colour ->)
    DrawColour: "draw" "color"  (io: r, g, b, a ->)
    DrawStroke: "draw" "stroke" (io: width ->)
    DrawFlush:  "drawflush"     (io: output ->)

    DrawRect:     "draw" "rect"     (io: x, y, w, h ->)
    DrawLineRect: "draw" "lineRect" (io: x, y, w, h ->)
    DrawPoly:     "draw" "poly"     (io: x, y, w, h ->)
    DrawLinePoly: "draw" "linePoly" (io: x, y, w, h ->)

    DrawTri:   "draw" "triangle" (io: x1, y1, x2, y2, x3, y3 ->)
    DrawImage: "draw" "image"    (io: x, y, image, size, rot ->)
    DrawPrint: "draw" "print"    (io: x, y, align ->)

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

    OpAdd:     "op" "add"  (oi: a, b -> c)
    OpSub:     "op" "sub"  (oi: a, b -> c)
    OpMul:     "op" "mul"  (oi: a, b -> c)
    OpDiv:     "op" "div"  (oi: a, b -> c)
    OpExp:     "op" "pow"  (oi: a, b -> c)
    OpIntDiv:  "op" "idiv" (oi: a, b -> c)
    OpMod:     "op" "mod"  (oi: a, b -> c)
    OpTrueMod: "op" "emod" (oi: a, b -> c)

    OpEq:            "op" "equal"         (oi: a, b -> result)
    OpStrictEq:      "op" "strictEqual"   (oi: a, b -> result)
    OpNotEqual:      "op" "notEqual"      (oi: a, b -> result)
    OpLAnd:          "op" "land"          (oi: a, b -> result)
    OpGreaterThan:   "op" "greaterThan"   (oi: a, b -> result)
    OpLessThan:      "op" "lessThan"      (oi: a, b -> result)
    OpGreaterThanEq: "op" "greaterThanEq" (oi: a, b -> result)
    OpLessThanEq:    "op" "lessThanEq"    (oi: a, b -> result)

    OpBAnd:    "op" "b-and" (oi: a, b -> result)
    OpOr:      "op" "or"    (oi: a, b -> result)
    OpXor:     "op" "xor"   (oi: a, b -> result)
    OpNot:     "op" "flip"  (oi: a, b -> result)
    OpLShift:  "op" "shl"   (oi: a, b -> result)
    OpRShift:  "op" "shr"   (oi: a, b -> result)
    OpURShift: "op" "ushr"  (oi: a, b -> result)

    OpMin: "op" "min" (oi: a, b -> result)
    OpMax: "op" "max" (oi: a, b -> result)

    OpAngle:     "op" "angle"     (oi: x, y -> result)
    OpAngleDiff: "op" "angleDiff" (oi: a, b -> result)
    OpLen:       "op" "len"       (oi: a, b -> result)

    OpRand:  "op" "rand"  (oi: d -> result)
    OpNoise: "op" "noise" (oi: x, y -> result)

    OpAbs:   "op" "abs"   (oi: x -> result)
    OpSign:  "op" "sign"  (oi: x -> result)
    OpFloor: "op" "floor" (oi: a, b -> result)
    OpCeil:  "op" "ceil"  (oi: x -> result)
    OpRound: "op" "round" (oi: x -> result)
    OpSqrt:  "op" "sqrt"  (oi: x -> result)

    OpLog:   "op" "log"   (oi: a, b -> result)
    OpLogN:  "op" "logn"  (oi: x -> result)
    OpLog10: "op" "log10" (oi: x -> result)

    OpSin:  "op" "sin"  (oi: x -> result)
    OpCos:  "op" "cos"  (oi: x -> result)
    OpTan:  "op" "tan"  (oi: x -> result)
    OpASin: "op" "asin" (oi: x -> result)
    OpACos: "op" "acos" (oi: x -> result)
    OpATan: "op" "atan" (oi: x -> result)

    UBind:   "ubind"   (io: unit_type ->)
    ULocate: "ulocate" (io: find, group, enemy, outx, outy -> found, building)

    UCIdle:     "ucontrol" "idle"     (oi: ->)
    UCStop:     "ucontrol" "stop"     (oi: ->)
    UCUnbind:   "ucontrol" "unbind"   (oi: ->)
    UCFlag:     "ucontrol" "flag"     (oi: flag ->)
    UCGetBlock: "ucontrol" "getBlock" (io: x, y -> building_type, building, floor_type)
    UCBuild:    "ucontrol" "build"    (oi: x, y, block, rotation, config ->)

    UCMove:     "ucontrol" "move"     (oi: x, y ->)
    UCPathfind: "ucontrol" "pathfind" (oi: x, y ->)
    UCApproach: "ucontrol" "approach" (oi: x, y, radius ->)
    UCWithin:   "ucontrol" "within"   (oi: x, y, radius -> result)
    UCBoost:    "ucontrol" "boost"    (oi: boost ->)
    UCMine:     "ucontrol" "mine"     (oi: x, y ->)

    UCTarget:   "ucontrol" "target"   (oi: x, y, shoot ->)
    UCTargetP:  "ucontrol" "targetp"  (oi: unit, shoot ->)

    UCPayTake:  "ucontrol" "payTake"  (oi: units ->)
    UCPayDrop:  "ucontrol" "payDrop"  (oi: ->)
    UCItemTake: "ucontrol" "itemTake" (oi: x, y, radius ->)
    UCItemDrop: "ucontrol" "itemDrop" (oi: to, amount ->)

    UCAutoPathfind: "ucontrol" "autoPathFind" (oi: ->)
    UCPayloadDrop:  "ucontrol" "payDrop"      (oi: ->)
    UCPayloadEnter: "ucontrol" "payEnter"     (oi: ->)
}

pub use thing::Statement;