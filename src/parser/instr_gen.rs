macro_rules! gen_instructions {
    (
        $name:ident,
        1input: $($ident_1i:ident ($($name_1i:literal)*) = $desc_1i:literal)*---
        2input: $($ident_2i:ident ($($name_2i:literal)*) = $desc_2i:literal)*---
    ) => {
        use std::collections::HashMap;
        use super::errs::StatementParseError;

        /// An mlog statement
        #[derive(Debug, PartialEq)]
        pub enum $name {
            /// A no-op
            Noop,

            /// A jump instruction
            Jump {
                /// The index to jump to
                index: usize,
                /// The condition
                cond: ConditionOp,
                /// The condition LHS
                lhs: Option<Argument>,
                /// The condition RHS
                rhs: Option<Argument>
            },
            $(
                #[doc = $desc_1i]
                $ident_1i {
                    /// The output variable name
                    o: String,
                    /// The input argument
                    i: Argument
                },

            )*
            $(
                #[doc = $desc_2i]
                $ident_2i {
                    /// The output variable name
                    c: String,
                    /// The LHS
                    a: Argument,
                    /// The RHS
                    b: Argument
                },
            )*
        }

        impl $name {
            /// Parse a set of whitespace split tokens into an instruction
            pub fn parse(v: &[&str], jump_labels: &HashMap<&str, usize>) -> Result<Self, StatementParseError> {
                match v {
                    ["jump", index, cond_str, lhs, rhs, ..] if ConditionOp::try_from(*cond_str).is_ok() => {
                        if let Ok(index) = index.parse() {
                            Ok($name::Jump {
                                index: index,
                                cond: ConditionOp::try_from(*cond_str).unwrap(),
                                lhs: Some(Argument::from(*lhs)),
                                rhs: Some(Argument::from(*rhs))
                            })
                        }
                        else {
                            Ok($name::Jump {
                                index: jump_labels
                                    .get(*index)
                                    .ok_or(StatementParseError::MissingJumpLabel(index.to_string()))?
                                    .clone(),
                                cond: ConditionOp::try_from(*cond_str).unwrap(),
                                lhs: Some(Argument::from(*lhs)),
                                rhs: Some(Argument::from(*rhs))
                            })
                        }
                    },
                    ["jump", index, "always", ..] => {
                        if let Ok(index) = index.parse() {
                            Ok($name::Jump {
                                index: index,
                                cond: ConditionOp::Always,
                                lhs: None,
                                rhs: None
                            })
                        }
                        else {
                            Ok($name::Jump {
                                index: jump_labels
                                    .get(*index)
                                    .ok_or(StatementParseError::MissingJumpLabel(index.to_string()))?
                                    .clone(),
                                cond: ConditionOp::Always,
                                lhs: None,
                                rhs: None
                            })
                        }
                    },
                    ["nop", ..] => Ok($name::Noop),
                    $([$($name_1i),*, o, i, ..] if matches!(Argument::from(*o), Argument::Variable(_)) => {
                        Ok($name::$ident_1i { o: o.to_string(), i: Argument::from(*i) })
                    },)*
                    $([$($name_2i),*, c, a, b, ..] if matches!(Argument::from(*c), Argument::Variable(_)) => {
                        Ok($name::$ident_2i { c: c.to_string(), a: Argument::from(*a), b: Argument::from(*b) })
                    },)*
                    _ => unimplemented!()
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $name::Jump { index, cond, lhs: None, rhs: None } => write!(f, "jump {} {}", index, cond),
                    $name::Jump { index, cond, lhs: Some(lhs), rhs: Some(rhs) } => write!(f, "jump {} {} {} {}", index, cond, lhs, rhs),
                    $name::Noop => write!(f, "nop"),
                    $($name::$ident_1i { o, i } => {
                        write!(f, "{} {} {}", concat!("" $(, $name_1i ,)" "*), o, i)
                    },)*
                    $($name::$ident_2i { c, b, a } => {
                        write!(f, "{} {} {} {}", concat!("" $(, $name_2i ,)" "*), c, b, a)
                    },)*
                    _ => unreachable!()
                }
            }
        }
    };
}

pub(super) use gen_instructions;
