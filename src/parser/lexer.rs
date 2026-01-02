use crate::parser::{errs::StatementParseError, statements::StatementType};
use regex::Regex;
use std::{collections::HashMap, marker::PhantomData, sync::LazyLock};

/// A lexer for mindustry logic.
/// 
/// # Examples
/// 
/// ```
/// # fn main() {
/// # use mlog_parse::parser::lexer::Lexer;
/// # use mlog_parse::parser::statements::Statement;
/// # use mlog_parse::parser::args::Argument;
/// # use mlog_parse::parser::args::ConditionOp;
/// const SRC: &str = r#"
///     loop_start:
///         op add i i 1
///         write i cell1 0
///     jump loop_start lessThan i 5
/// "#;
///
/// let lexer: Lexer<Statement> = Lexer::new(SRC);
/// 
/// let instructions: Vec<_> = lexer
///     .map(|x| x.unwrap())
///     .collect();
///
/// assert_eq!(
///     instructions,
///     vec![
///         Statement::OpAdd{
///             a: Argument::Variable("i"),
///             b: Argument::Number(1.),
///             c: "i"
///         },
///         Statement::Write {
///             value: Argument::Variable("i"),
///             cell:  Argument::Variable("cell1"),
///             index: Argument::Number(0.)
///         },
///         Statement::Jump {
///            index: 0,
///            cond: ConditionOp::LessThan,
///            lhs: Some(Argument::Variable("i")),
///            rhs: Some(Argument::Number(5.)),
///         }
///     ]
/// );
/// # }
/// ```
pub struct Lexer<'a, T: StatementType<'a>> {
    lines: Vec<(usize, &'a str)>,
    jump_labels: HashMap<&'a str, usize>,
    index: usize,
    _marker: PhantomData<T>,
}

static JUMPLABEL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*?[[:word:]]+:").unwrap());
static COMMENT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*#").unwrap());

impl<'a, T: StatementType<'a>> Lexer<'a, T> {
    /// Renames synonymous tokens as necessary
    fn do_renaming(tokens: &[&'a str]) -> Vec<&'a str> {
        match tokens {
            ["op", "and", rest @ ..] => {
                let mut vec = vec!["op", "b-and"];
                vec.extend(rest);
                vec
            }
            ["op", "lor", rest @ ..] => {
                let mut vec = vec!["op", "or"];
                vec.extend(rest);
                vec
            }
            ["op", "not", rest @ ..] => {
                let mut vec = vec!["op", "flip"];
                vec.extend(rest);
                vec
            }
            ["noop", ..] => {
                vec!["nop"]
            }
            x => x.into(),
        }
    }

    /// Creates a new lexer.
    #[must_use]
    pub fn new(str: &'a str) -> Self {
        let mut lines = Vec::new();
        let mut jump_labels = HashMap::new();
        for (idx, line) in str
            .lines()
            .enumerate()
            .filter(|(_, x)| x.contains(|x: char| !x.is_whitespace()))
        {
            if COMMENT_REGEX.is_match(line) {
            } else if JUMPLABEL_REGEX.is_match(line) {
                jump_labels.insert(line.trim().strip_suffix(":").unwrap(), idx - 1);
            } else {
                lines.push((idx, line));
            }
        }

        Self {
            lines,
            jump_labels,
            index: 0,
            _marker: PhantomData {},
        }
    }
}

impl<'a, T: StatementType<'a>> Iterator for Lexer<'a, T> {
    type Item = Result<T, StatementParseError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, line) = self.lines.get(self.index)?;
        let split: Vec<_> = line.split_whitespace().collect();

        self.index += 1;

        Some(T::try_parse(&Self::do_renaming(&split), &self.jump_labels))
    }
}
