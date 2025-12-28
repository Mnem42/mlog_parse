macro_rules! gen_instructions {
    (
        $name:ident,
        0i0o: $($ident_0i0o:ident ($($name_0i0o:literal)*) = $desc_0i0o:literal)*---
        1i0o: $($ident_1i0o:ident ($($name_1i0o:literal)*) = $desc_1i0o:literal)*---
        2i0o: $($ident_2i0o:ident ($($name_2i0o:literal)*) = $desc_2i0o:literal)*---

        1i1o: $($ident_1i1o:ident ($($name_1i1o:literal)*) = $desc_1i1o:literal)*---
        2i1o: $($ident_2i1o:ident ($($name_2i1o:literal)*) = $desc_2i1o:literal)*---
    ) => {
        use std::collections::HashMap;
        use super::errs::StatementParseError;

        /// An mlog statement
        #[derive(Debug, PartialEq)]
        #[allow(clippy::empty_docs)]
        pub enum $name<'a> {
            /// A jump instruction
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
            $(
                #[doc = $desc_0i0o]
                $ident_0i0o,
            )*
            $(
                #[doc = $desc_1i0o]
                $ident_1i0o {
                    /// The argument
                    arg: Argument<'a>
                },
            )*
            $(
                #[doc = $desc_1i1o]
                $ident_1i1o {
                    /// The output variable name
                    o: String,
                    /// The input argument
                    i: Argument<'a>
                },
            )*
            $(
                #[doc = $desc_2i0o]
                $ident_2i0o {
                    /// Argument A
                    a: Argument<'a>,
                    /// Argument B
                    b: Argument<'a>
                },
            )*
            $(
                #[doc = $desc_2i1o]
                $ident_2i1o {
                    /// The output variable name
                    c: String,
                    /// The LHS
                    a: Argument<'a>,
                    /// The RHS
                    b: Argument<'a>
                },
            )*
        }

        impl<'a> $name<'a> {
            /// Parse a set of whitespace split tokens into an instruction
            #[allow(unreachable_patterns)]
            pub fn parse(v: &[&'a str], jump_labels: &HashMap<&str, usize>) -> Result<Self, StatementParseError> {
                match v {
                    ["jump", index, cond_str, lhs, rhs, ..] if ConditionOp::try_from(*cond_str).is_ok() => {
                        if let Ok(index) = index.parse() {
                            Ok($name::Jump {
                                index,
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
                                index,
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
                    $([$($name_2i1o),*, c, a, b, ..] if matches!(Argument::from(*c), Argument::Variable(_)) => {
                        Ok($name::$ident_2i1o { c: c.to_string(), a: Argument::from(*a), b: Argument::from(*b) })
                    },)*
                    $([$($name_1i1o),*, o, i, ..] if matches!(Argument::from(*o), Argument::Variable(_)) => {
                        Ok($name::$ident_1i1o { o: o.to_string(), i: Argument::from(*i) })
                    },)*
                    $([$($name_2i0o),*, a, b, ..] => {
                        Ok($name::$ident_2i0o { a: Argument::from(*a), b: Argument::from(*b) })
                    },)*
                    $([$($name_1i0o),*, arg, ..] => {
                        Ok($name::$ident_1i0o { arg: Argument::from(*arg) })
                    },)*
                    $([$($name_0i0o),*, ..] => {
                        Ok($name::$ident_0i0o {})
                    },)*
                    _ => unimplemented!()
                }
            }
        }

        impl fmt::Display for $name<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $name::Jump { index, cond, lhs: None, rhs: None } => write!(f, "jump {} {}", index, cond),
                    $name::Jump { index, cond, lhs: Some(lhs), rhs: Some(rhs) } => write!(f, "jump {} {} {} {}", index, cond, lhs, rhs),
                    $($name::$ident_0i0o => {
                        write!(f, "{}", concat!("" $(, $name_0i0o ,)" "*))
                    },)*
                    $($name::$ident_1i0o { arg } => {
                        write!(f, "{} {}", concat!("" $(, $name_1i0o ,)" "*), arg)
                    },)*
                    $($name::$ident_2i0o { a, b } => {
                        write!(f, "{} {} {}", concat!("" $(, $name_2i0o ,)" "*), a, b)
                    },)*
                    $($name::$ident_1i1o { o, i } => {
                        write!(f, "{} {} {}", concat!("" $(, $name_1i1o ,)" "*), o, i)
                    },)*
                    $($name::$ident_2i1o { c, b, a } => {
                        write!(f, "{} {} {} {}", concat!("" $(, $name_2i1o ,)" "*), c, b, a)
                    },)*
                    _ => unreachable!()
                }
            }
        }
    };
}

pub(super) use gen_instructions;
