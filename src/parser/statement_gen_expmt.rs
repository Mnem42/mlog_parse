macro_rules! gen_match_l {
    ($($name:literal),* $($l:ident),* -> $($r:ident),*) => { [$($name),*, $($l,)* $($r,)* ..] }
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
        $enum: ident, 
        $($ident:ident: ($($i:ident),* -> $($o:ident),*))*
    ) => {
        /// A statement
        #[derive(Debug, PartialEq)]
        pub enum $enum<'a> {
            $($ident {
                $($i: Argument<'a>,)*
                $($o: &'a str),*
            }),*,
            Jump {
                /// The index to jump to
                index: usize,
                /// The condition
                cond: ConditionOp,
                /// The condition LHS
                lhs: Option<Argument<'a>>,
                /// The condition RHS
                rhs: Option<Argument<'a>>
            }
        }
    };
}

/// Generates a statements enum
/// 
/// `oi` means the outputs are *before* the inputs in the statement, while `io` means the inputs
/// are *before* the outputs in the statement.
macro_rules! gen_statements {
    {
        $enum: ident, 
        oi: $(
            $oi_ident:ident:
            $($oi_name:literal)*
            ($($oi_i:ident),* -> $($oi_o:ident),*)
        )*
        ---
        io: $(
            $io_ident:ident:
            $($io_name:literal)*
            ($($io_i:ident),* -> $($io_o:ident),*)
        )*
        ---
    } => {mod thing {
        use crate::parser::instructions::Argument;
        use crate::parser::errs::StatementParseError;
        use crate::parser::instructions::ConditionOp;

        gen_enum!{
            $enum,
            $($oi_ident: ($($oi_i),* -> $($oi_o),*))*
            $($io_ident: ($($io_i),* -> $($io_o),*))*
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
                        gen_match_l!($($oi_name),* $($oi_o),* -> $($oi_i),*)
                            if gen_match_guard!($($oi_o)*)
                        => Ok(gen_match_result!($enum $oi_ident $($oi_i),* -> $($oi_o),*)),
                    )*
                    $(
                        gen_match_l!($($io_name),* $($io_i),* -> $($io_o),*)
                            if gen_match_guard!($($io_o)*) 
                        => Ok(gen_match_result!($enum $io_ident $($io_i),* -> $($io_o),*)),
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
                        Self::$oi_ident {$($oi_i),* $(, $oi_o)*} => {
                            f.write_str(concat!("" $(, $oi_name ,)" "*));
                            $($oi_o.fmt(f);)*
                            $($oi_i.fmt(f);)*
                            
                            Ok(())
                        },
                    )*
                    $(
                        Self::$io_ident {$($io_i),* $(,$io_o)*} => {
                            f.write_str(concat!("" $(, $io_name ,)" "*));
                            $($io_o.fmt(f);)*
                            $($io_i.fmt(f);)*
                            
                            Ok(())
                        },
                    )*
                    _ => unreachable!()
                }
            }
        }
    }
}}


mod x {
    gen_statements!{
        Statement,

        oi:
            Set: "set" (value -> var)
            
            OpAdd: "op" "add" (a, b -> c)
            OpSub: "op" "sub" (a, b -> c)
            OpMul: "op" "mul" (a, b -> c)
            OpDiv: "op" "div" (a, b -> c)
        ---

        io: 
            Noop: "nop" (->)
            ULocate: "ulocate" (find, group, enemy, outx, outy -> found, building)
        ---
    }
}