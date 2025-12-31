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
            $($o),*
            $(, $i: Argument::from(*$i))*
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

    Noop: "nop" (oi: ->)

    Set: "set" (oi: value -> var)
    
    OpAdd: "op" "add" (oi: a, b -> c)
    OpSub: "op" "sub" (oi: a, b -> c)
    OpMul: "op" "mul" (oi: a, b -> c)
    OpDiv: "op" "div" (oi: a, b -> c)

    ULocate: "ulocate" (io: find, group, enemy, outx, outy -> found, building)
}

pub use thing::Statement;