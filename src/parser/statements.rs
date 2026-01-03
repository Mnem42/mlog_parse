//! This module defines the [`Statement`] and [`WprocStatement`] types

macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
}

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
        $enum:ident ($docs:literal)
        $($ident:ident $($i:ident),* -> $($o:ident),*);*
    ) => {
        #[derive(Debug, PartialEq, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[doc = $docs]
        pub enum $enum<'a> {
            /// A jump statement
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
            /// A select statement
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

            $(
                // There just isn't much of a point in adding doc support
                #[allow(missing_docs)]
                $ident {
                    $($i: Argument<'a>,)*
                    $($o: &'a str),*
                }
            ),*,
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

macro_rules! impl_statement {
    (
        $enum:ident
        $(
            $ident:ident:
            $($name:literal)*
            ($ty:tt: $($i:ident),* -> $($o:ident),*)
        )*
    ) => {
        impl<'a> $enum<'a> {
            /// Gets the number of operands that can be passed to a statement.
            ///
            /// # Panics
            ///
            /// This function panics if a jump or select statement that is less than 3 items long is
            /// passed in that has less than 3 elements. This isn't validated since it couldn't
            /// possibly be matched.
            fn operand_count(tokens: &[&'a str]) -> usize {
                match tokens {
                    // These are based on the longest possible invocation
                    ["jump", ..]   => if tokens[2] == "always" { 3 } else { 5 },
                    ["select", ..] => if tokens[2] == "always" { 5 } else { 7 },
                    $(
                        [$($name),*, ..] => { count_tts!($($name)* $($i:ident)* $($o:ident)*) }
                    )*
                    _ => 0
                }
            }
        }

        impl<'a> StatementType<'a> for $enum<'a> {
            /// Tries to parse a token.
            ///
            /// # Errors
            ///
            /// If the jump label a `jump` statement points to isn't found in the jump_labels
            /// parameter or an invalid statement is passed in, an error variant is returned.
            fn try_parse(
                tokens: &[&'a str],
                jump_labels: &std::collections::HashMap<&'a str, usize>
            ) -> Result<Self, super::ParseError<'a>> {
                let mut padded_tokens = tokens.to_vec();
                padded_tokens.resize(Self::operand_count(tokens), "0");

                match padded_tokens.as_slice() {
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
                                    .ok_or(super::ParseError::MissingJumpLabel(index))?
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
                                    .ok_or(super::ParseError::MissingJumpLabel(index))?
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
                    _ => Err(super::ParseError::InvalidInstruction(tokens.to_vec()))
                }
            }
        }
    }
}

/// Generates a statements enum
///
/// `oi` means the outputs are *before* the inputs in the statement, while `io` means the inputs
/// are *before* the outputs in the statement.
macro_rules! gen_statements {
    {
        normal_enum: $enum:ident      $normal_docs:literal
        wproc_enum: $wproc_enum:ident $wproc_docs:literal
        normal:
            $(
                $ident:ident:
                $($name:literal)*
                ($ty:tt: $($i:ident),* -> $($o:ident),*)
            )*
        ---
        wproc:
            $(
                $wp_ident:ident:
                $($wp_name:literal)*
                ($wp_ty:tt: $($wp_i:ident),* -> $($wp_o:ident),*)
            )*
        ---
    } => {mod thing {
        use crate::parser::args::Argument;
        use crate::parser::args::ConditionOp;
        use crate::parser::statements::StatementType;

        gen_enum!{
            $enum ($normal_docs)
            $($ident $($i),* -> $($o),*);*
        }

        gen_enum!{
            $wproc_enum ($wproc_docs)
            $($ident $($i),* -> $($o),*);*;
            $($wp_ident $($wp_i),* -> $($wp_o),*);*
        }

        impl_statement!{
            $enum
            $(
                $ident:
                $($name)*
                ($ty: $($i),* -> $($o),*)
            )*
        }
        impl_statement!{
            $wproc_enum
            $(
                $ident:
                $($name)*
                ($ty: $($i),* -> $($o),*)
            )*
            $(
                $wp_ident:
                $($wp_name)*
                ($wp_ty: $($wp_i),* -> $($wp_o),*)
            )*
        }

        impl std::fmt::Display for $enum<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    Self::Jump { index, cond, lhs: None, rhs: None } =>
                        write!(f, "jump {} {}", index, cond),
                    Self::Jump { index, cond, lhs: Some(lhs), rhs: Some(rhs) } =>
                        write!(f, "jump {} {} {} {}", index, cond, lhs, rhs),
                    Self::Select { result, cond, lhs: None, rhs: None, true_option, false_option } =>
                        write!(f, "select {} {} {} {}", result, cond, true_option, false_option),
                    Self::Select { result, cond, lhs: Some(lhs), rhs: Some(rhs), true_option, false_option } =>
                        write!(f, "select {} {} {} {} {} {}", result, cond, lhs, rhs, true_option, false_option),

                    // Other combinations should be impossible
                    Self::Jump {..} | Self::Select {..} => unreachable!(),
                    $(
                        Self::$ident {$($i),* $(,$o)*} => {
                            gen_printer!($ty f ; $($name),* $($i),* -> $($o),*)
                        },
                    )*
                }
            }
        }

        // Not really worth making another enum for this
        impl std::fmt::Display for $wproc_enum<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    Self::Jump { index, cond, lhs: None, rhs: None } =>
                        write!(f, "jump {} {}", index, cond),
                    Self::Jump { index, cond, lhs: Some(lhs), rhs: Some(rhs) } =>
                        write!(f, "jump {} {} {} {}", index, cond, lhs, rhs),
                    Self::Select { result, cond, lhs: None, rhs: None, true_option, false_option } =>
                        write!(f, "select {} {} {} {}", result, cond, true_option, false_option),
                    Self::Select { result, cond, lhs: Some(lhs), rhs: Some(rhs), true_option, false_option } =>
                        write!(f, "select {} {} {} {} {} {}", result, cond, lhs, rhs, true_option, false_option),

                    // Other combinations should be impossible
                    Self::Jump {..} | Self::Select {..} => unreachable!(),
                    $(
                        Self::$ident {$($i),* $(,$o)*} => {
                            gen_printer!($ty f ; $($name),* $($i),* -> $($o),*)
                        },
                    )*
                    $(
                        Self::$wp_ident {$($wp_i),* $(,$wp_o)*} => {
                            gen_printer!($wp_ty f ; $($wp_name),* $($wp_i),* -> $($wp_o),*)
                        },
                    )*
                }
            }
        }
    }
}}
use std::collections::HashMap;
use std::fmt::Display;

/// An error found when parsing a statement
#[derive(Debug, PartialEq)]
pub enum ParseError<'s> {
    /// Missing jump label
    MissingJumpLabel(&'s str),
    /// Invalid instruction
    InvalidInstruction(Vec<&'s str>),
}

/// Trait for anything that can be used as a statement
pub trait StatementType<'a>: Display + Sized {
    /// Parses a statement
    fn try_parse(
        tokens: &[&'a str],
        jump_labels: &HashMap<&'a str, usize>,
    ) -> Result<Self, ParseError<'a>>;
}

gen_statements! {
    normal_enum: Statement      "A statement for a normal logic processor."
    wproc_enum:  WprocStatement "A statement for a world processor."

    normal:
        Noop: "nop"  (io: ->)
        Stop: "stop" (io: ->)
        End:  "end"  (io: ->)
        Set:  "set"  (oi: value -> var)
        Wait: "wait" (io: time ->)

        GetLink: "getlink" (oi: index -> result)
        Radar:   "radar"   (io: m1, m2, m3, sort, block -> result)
        Sensor:  "sensor"  (oi: item -> result)

        Read:  "read"  (oi: cell, index -> result)
        Write: "write" (io: value, cell, index ->)

        Print:        "print"           (oi: text ->)
        PrintChar:    "printchar"       (oi: char ->)
        Format:       "format"          (oi: f_string ->)
        PrintFlush:   "printflush"      (oi: output ->)

        PackColour:   "packcolor"       (oi: r, g, b, a -> result)
        UnpackColour: "unpackcolor"     (oi: input -> r, g, b, a)
        DrawReset:    "draw" "reset"    (oi: ->)
        DrawClear:    "draw" "clear"    (oi: r, g, b ->)
        DrawCol:      "draw" "col"      (oi: packed_colour ->)
        DrawColour:   "draw" "color"    (oi: r, g, b, a ->)
        DrawStroke:   "draw" "stroke"   (oi: width ->)
        DrawFlush:    "drawflush"       (oi: output ->)

        DrawRect:     "draw" "rect"     (oi: x, y, w, h ->)
        DrawLineRect: "draw" "lineRect" (oi: x, y, w, h ->)
        DrawPoly:     "draw" "poly"     (oi: x, y, w, h ->)
        DrawLinePoly: "draw" "linePoly" (oi: x, y, w, h ->)
        DrawLine:     "draw" "line"     (oi: x, y, x2, y2 ->)

        DrawTri:   "draw" "triangle" (io: x1, y1, x2, y2, x3, y3 ->)
        DrawImage: "draw" "image"    (io: x, y, image, size, rot ->)
        DrawPrint: "draw" "print"    (io: x, y, align ->)

        DrawTranslate:  "draw" "translate" (oi: x, y ->)
        DrawRotate:     "draw" "rotate"    (oi: angle ->)
        DrawScale:      "draw" "scale"     (oi: x, y ->)

        ControlEnabled: "control" "enabled" (oi: enabled ->)
        ControlConfig:  "control" "config"  (oi: config ->)
        ControlColour:  "control" "color"   (oi: colour ->)
        ControlShoot:   "control" "shoot"   (oi: block, x, y, shoot ->)
        ControlShootP:  "control" "shootp"  (oi: block, unit, shoot ->)

        BlockLookup:    "lookup" "block"  (oi: index -> result)
        UnitLookup:     "lookup" "unit"   (oi: index -> result)
        ItemLookup:     "lookup" "item"   (oi: index -> result)
        LiquidLookup:   "lookup" "liquid" (oi: index -> result)
        TeamLookup:     "lookup" "team"   (oi: index -> result)

        OpAdd:     "op" "add"  (oi: a, b -> c)
        OpSub:     "op" "sub"  (oi: a, b -> c)
        OpMul:     "op" "mul"  (oi: a, b -> c)
        OpDiv:     "op" "div"  (oi: a, b -> c)
        OpExp:     "op" "pow"  (oi: a, b -> c)
        OpIntDiv:  "op" "idiv" (oi: a, b -> c)
        OpMod:     "op" "mod"  (oi: a, b -> c)
        OpTrueMod: "op" "emod" (oi: a, b -> c)

        OpEq:             "op" "equal"          (oi: a, b -> result)
        OpStrictEq:       "op" "strictEqual"    (oi: a, b -> result)
        OpNotEqual:       "op" "notEqual"       (oi: a, b -> result)
        OpStrictNotEqual: "op" "strictNotEqual" (oi: a, b -> result)
        OpLAnd:           "op" "land"           (oi: a, b -> result)
        OpGreaterThan:    "op" "greaterThan"    (oi: a, b -> result)
        OpLessThan:       "op" "lessThan"       (oi: a, b -> result)
        OpGreaterThanEq:  "op" "greaterThanEq"  (oi: a, b -> result)
        OpLessThanEq:     "op" "lessThanEq"     (oi: a, b -> result)

        OpBAnd:    "op" "b-and" (oi: a, b -> result)
        OpOr:      "op" "or"    (oi: a, b -> result)
        OpXor:     "op" "xor"   (oi: a, b -> result)
        OpNot:     "op" "flip"  (oi: a, b -> result)
        OpLShift:  "op" "shl"   (oi: a, b -> result)
        OpRShift:  "op" "shr"   (oi: a, b -> result)
        OpURShift: "op" "ushr"  (oi: a, b -> result)

        OpMin:  "op" "min" (oi: a, b -> result)
        OpMax:  "op" "max" (oi: a, b -> result)

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

        OpSin:   "op" "sin"  (oi: x -> result)
        OpCos:   "op" "cos"  (oi: x -> result)
        OpTan:   "op" "tan"  (oi: x -> result)
        OpASin:  "op" "asin" (oi: x -> result)
        OpACos:  "op" "acos" (oi: x -> result)
        OpATan:  "op" "atan" (oi: x -> result)

        UBind:   "ubind"   (io: unit_type ->)
        ULocate: "ulocate" (io: find, group, enemy, outx, outy -> found, building)
        URadar:  "uradar"  (io: m1, m2, m3, sort, block -> result)

        UCIdle:         "ucontrol" "idle"         (oi: ->)
        UCStop:         "ucontrol" "stop"         (oi: ->)
        UCUnbind:       "ucontrol" "unbind"       (oi: ->)
        UCFlag:         "ucontrol" "flag"         (oi: flag ->)
        UCGetBlock:     "ucontrol" "getBlock"     (io: x, y -> building_type, building, floor_type)
        UCBuild:        "ucontrol" "build"        (oi: x, y, block, rotation, config ->)
        UCDeconstruct:  "ucontrol" "deconstruct"  (oi: x, y ->)
        UCMove:         "ucontrol" "move"         (oi: x, y ->)
        UCPathfind:     "ucontrol" "pathfind"     (oi: x, y ->)
        UCAutoPathfind: "ucontrol" "autoPathfind" (oi: ->)
        UCApproach:     "ucontrol" "approach"     (oi: x, y, radius ->)
        UCWithin:       "ucontrol" "within"       (oi: x, y, radius -> result)
        UCBoost:        "ucontrol" "boost"        (oi: boost ->)
        UCMine:         "ucontrol" "mine"         (oi: x, y ->)
        UCTarget:       "ucontrol" "target"       (oi: x, y, shoot ->)
        UCTargetP:      "ucontrol" "targetp"      (oi: unit, shoot ->)
        UCItemTake:     "ucontrol" "itemTake"     (oi: x, y, radius ->)
        UCItemDrop:     "ucontrol" "itemDrop"     (oi: to, amount ->)
        UCPayloadTake:  "ucontrol" "payTake"      (oi: units ->)
        UCPayloadDrop:  "ucontrol" "payDrop"      (oi: ->)
        UCPayloadEnter: "ucontrol" "payEnter"     (oi: ->)
    ---

    wproc:
        GetBlock:     "getblock"         (oi: x, y -> result)
        SetBOre:      "setblock" "ore"   (oi: x, y, to ->)
        SetBFloor:    "setblock" "floor" (oi: x, y, to ->)
        SetBBlock:    "setblock" "block" (oi: x, y, to, team, rotation ->)
        SetProp:      "setprop"          (oi: prop, block, value ->)
        ShowMessage:  "message"          (oi: msg_type, duration, success ->)
        WeatherSense: "weathersense"     (oi: weather -> result)
        WeatherSet:   "weatherset"       (oi: weather, state ->)
        SetRate:      "setrate"          (oi: rate ->)
        Sync:         "sync"             (oi: var ->)
        GetFlag:      "getflag"          (oi: flag -> value)
        SetFlag:      "setflag"          (oi: value -> flag)
        LocalePrint:  "localeprint"      (oi: property ->)

        CutsceneStop: "cutscene" "stop" (oi: ->)
        CutsceneZoom: "cutscene" "zoom" (oi: level ->)
        CutscenePan:  "cutscene" "pan"  (oi: x, y, speed ->)

        FetchUnit:        "fetch" "unit"        (oi: team, number, unit -> result)
        FetchUnitCount:   "fetch" "unitCount"   (oi: team, unit -> result)
        FetchPlayer:      "fetch" "player"      (oi: team, number -> result)
        FetchPlayerCount: "fetch" "playerCount" (oi: team -> result)
        FetchCore:        "fetch" "core"        (oi: team, number -> result)
        FetchCoreCount:   "fetch" "coreCount"   (oi: team -> result)
        FetchBuild:       "fetch" "build"       (oi: team, number, block -> result)
        FetchBuildCount:  "fetch" "buildCount"  (oi: team, block -> result)

        PlaySoundPositional: "playsound" "false" (oi: sound, volume, pitch, x, y, _1, limit ->)
        PlaySoundGlobal:     "playsound" "true"  (oi: sound, volume, pitch, _1, _2, pan, limit ->)
        Explosion:           "explosion" (oi: team, x, y, radius, damage, air, ground, pierce, effect ->)

        SpawnUnit:    "spawn"        (io: unit_type, x, y, rotation, team -> result)
        ApplyStatus:  "status"       (oi: _padding, wet, unit, duration ->)
        SpawnWave:    "spawnwave"    (oi: natural, x, y ->)

        SetRCurrentWaveTime:      "setrule" "currentWaveTime"      (oi: v ->)
        SetRWaveTimer:            "setrule" "waveTimer"            (oi: v ->)
        SetRWaves:                "setrule" "waves"                (oi: v ->)
        SetRWave:                 "setrule" "wave"                 (oi: v ->)
        SetRWaveSpacing:          "setrule" "waveSpacing"          (oi: v ->)
        SetRWaveSending:          "setrule" "waveSending"          (oi: v ->)
        SetRAttackMode:           "setrule" "attackMode"           (oi: v ->)
        SetREnemyCoreBuildRadius: "setrule" "enemyCoreBuildRadius" (oi: v ->)
        SetRDropZoneRadius:       "setrule" "dropZoneRadius"       (oi: v ->)
        SetRUnitCap:              "setrule" "unitCap"              (oi: v ->)
        SetRLighting:             "setrule" "lighting"             (oi: v ->)
        SetRCanGameOver:          "setrule" "canGameOver"          (oi: v ->)
        SetRAmbientLight:         "setrule" "ambientLight"         (oi: v ->)
        SetRSolarMultiplier:      "setrule" "solarMultiplier"      (oi: multiplier ->)
        SetRDragMultiplier:       "setrule" "dragMultiplier"       (oi: multiplier ->)
        SetRBan:                  "setrule" "ban"                  (oi: index ->)
        SetRUnban:                "setrule" "unban"                (oi: index ->)

        SetRBuildSpeed:     "setrule" "buildSpeed"     (oi: team, v ->)
        SetRUnitHealth:     "setrule" "unitHealth"     (oi: team, v ->)
        SetRUnitBuildSpeed: "setrule" "unitBuildSpeed" (oi: team, v ->)
        SetRUnitMineSpeed:  "setrule" "unitMineSpeed"  (oi: team, v ->)
        SetRUnitCost:       "setrule" "unitCost"       (oi: team, v ->)
        SetRUnitDamage:     "setrule" "unitDamage"     (oi: team, v ->)
        SetRBlockHealth:    "setrule" "blockHealth"    (oi: team, v ->)
        SetRBlockDamage:    "setrule" "blockDamage"    (oi: team, v ->)
        SetRRtsMinWeight:   "setrule" "rtsMinWeight"   (oi: team, v ->)
        SetRRtsMinSquad:    "setrule" "rtsMinSquad"    (oi: team, v ->)
        SetRMapArea:        "setrule" "mapArea"        (oi: x, y, w, h ->)

        EffectWarn:        "effect" "warn"            (oi: x, y ->)
        EffectCross:       "effect" "cross"           (oi: x, y ->)
        EffectSpawn:       "effect" "spawn"           (oi: x, y ->)
        EffectTrail:       "effect" "trail"           (oi: x, y, colour, size ->)
        EffectBreakProp:   "effect" "breakProp"       (oi: x, y, colour, size ->)
        EffectSmokeCloud:  "effect" "smokeCloud"      (oi: x, y, colour ->)
        EffectVapour:      "effect" "vapor"           (oi: x, y, colour ->)
        EffectHit:         "effect" "hit"             (oi: x, y, colour ->)
        EffectHitSquare:   "effect" "hitSquare"       (oi: x, y, colour ->)
        EffectWave:        "effect" "wave"            (oi: x, y, colour, size ->)
        EffectBubble:      "effect" "bubble"          (oi: x, y ->)
        EffectSmokePuff:   "effect" "smokePuff"       (oi: x, y, colour ->)

        EffectBlockFall:   "effect" "blockFall"       (oi: x, y, data ->)
        EffectPlaceBlock:  "effect" "placeBlock"      (oi: x, y, size ->)
        EffectPlaceBlockS: "effect" "placeBlockSpark" (oi: x, y, size ->)
        EffectBreakBlock:  "effect" "breakBlock"      (oi: x, y, size ->)
        EffectLightBlock:  "effect" "lightBlock"      (oi: x, y, colour, size ->)

        EffectShootBig:    "effect" "shootBig"        (oi: x, y, colour, rotation ->)
        EffectShootSmall:  "effect" "shootSmall"      (oi: x, y, colour, rotation ->)

        EffectSmokeSmall:     "effect" "smokeSmall"      (oi: x, y, rotation ->)
        EffectSmokeBig:       "effect" "smokeBig"        (oi: x, y, rotation ->)
        EffectSmokeColour:    "effect" "smokeColor"      (oi: x, y, colour, rotation ->)
        EffectSmokeSquare:    "effect" "smokeSquare"     (oi: x, y, colour, rotation ->)
        EffectSmokeSquareBig: "effect" "smokeSquareBig"  (oi: x, y, colour, rotation ->)

        EffectSpark:          "effect" "spark"           (oi: x, y, rotation ->)
        EffectSparkBig:       "effect" "sparkBig"        (oi: x, y, rotation ->)
        EffectSparkShoot:     "effect" "sparkShoot"      (oi: x, y, colour, rotation ->)
        EffectSparkShootBig:  "effect" "sparkShootBig"   (oi: x, y, colour, rotation ->)

        EffectDrill:          "effect" "drill"    (oi: x, y, colour ->)
        EffectDrillBig:       "effect" "drillBig" (oi: x, y, colour ->)

        EffectExplosion:      "effect" "explosion"      (oi: x, y, size ->)
        EffectSparkExplosion: "effect" "sparkExplosion" (oi: x, y, colour ->)
        EffectCrossExplosion: "effect" "crossExplosion" (oi: x, y, colour, size ->)

        MakeMarkerShapeText: "makemarker" "shapeText"  (oi: id, x, y, replace ->)
        MakeMarkerPoint:     "makemarker" "point"      (oi: id, x, y, replace ->)
        MakeMarkerShape:     "makemarker" "shape"      (oi: id, x, y, replace ->)
        MakeMarkerText:      "makemarker" "text"       (oi: id, x, y, replace ->)
        MakeMarkerLine:      "makemarker" "line"       (oi: id, x, y, replace ->)
        MakeMarkerTexture:   "makemarker" "texture"    (oi: id, x, y, replace ->)
        MakeMarkerQuad:      "makemarker" "quad"       (oi: id, x, y, replace ->)

        SetMarkerRemove:     "setmarker" "remove"      (oi: id ->)
        SetMarkerWorld:      "setmarker" "world"       (oi: id, val ->)
        SetMarkerMinimap:    "setmarker" "minimap"     (oi: id, val ->)
        SetMarkerDrawLayer:  "setmarker" "drawLayer"   (oi: id, layer ->)
        SetMarkerAutoscale:  "setmarker" "autoscale"   (oi: id, val ->)
        SetMarkerPos:        "setmarker" "pos"         (oi: id, x, y ->)
        SetMarkerEndPos:     "setmarker" "endPos"      (oi: id, x, y ->)
        SetMarkerLayer:      "setmarker" "layer"       (oi: id, val ->)
        SetMarkerColour:     "setmarker" "color"       (oi: id, val ->)
        SetMarkerStroke:     "setmarker" "stroke"      (oi: id, val ->)
        SetMarkerOutline:    "setmarker" "outline"     (oi: id, val ->)
        SetMarkerRadius:     "setmarker" "radius"      (oi: id, val ->)
        SetMarkerRotation:   "setmarker" "rotation"    (oi: id, val ->)
        SetMarkerShape:      "setmarker" "shape"       (oi: id, sides, fill, radius ->)
        SetMarkerArc:        "setmarker" "arc"         (oi: id, start, end ->)
        SetMarkerFontSize:   "setmarker" "fontSize"    (oi: id, size ->)
        SetMarkerTextHeight: "setmarker" "textHeight"  (oi: id, height ->)
        SetMarkerTextAlign:  "setmarker" "textAlign"   (oi: id, align ->)
        SetMarkerLineAlign:  "setmarker" "lineAlign"   (oi: id, align ->)
        SetMarkerFlushText:  "setmarker" "flushText"   (oi: id, fetch ->)
        SetMarkerLabelFlags: "setmarker" "labelFlags"  (oi: id, background, outline ->)
        SetMarkerTexture:    "setmarker" "texture"     (oi: id, printflush, name ->)
        SetMarkerTexSize:    "setmarker" "textureSize" (oi: id, width, height ->)
        SetMarkerPosI:       "setmarker" "posi"        (oi: id, index, x, y ->)
        SetMarkerUVI:        "setmarker" "uvi"         (oi: id, index, x, y ->)
        SetMarkerColourI:    "setmarker" "colori"      (oi: id, index, colour ->)
    ---
}

pub use thing::{Statement, WprocStatement};
