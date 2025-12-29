use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

/// Error handling
pub mod errs;
mod statement_gen;
/// Instructions
pub mod instructions;
/// The definition of the Statement type.
pub mod statement_def;

use errs::StatementParseError;
use statement_def::Statement;

/// A lexer for mindustry logic.
pub struct Lexer<'a> {
    lines: Vec<(usize, &'a str)>,
    jump_labels: HashMap<&'a str, usize>,
    index: usize,
}

/// Parse a literal with a prefix (e.g. 0x05) with a given radix.
fn parse_nradix_literal(text: &str, radix: u32) -> i64 {
    let mut chars = text.chars();

    match chars.next().unwrap() {
        sign @ ('+' | '-') => {
            i64::from_str_radix(&text[3..], radix).unwrap() * if sign == '-' { -1 } else { 1 }
        }
        _ => i64::from_str_radix(&text[2..], radix).unwrap(),
    }
}

static JUMPLABEL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*?[[:word:]]+:").unwrap());
static COMMENT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*#").unwrap());

impl<'a> Lexer<'a> {
    /// Create a new lexer
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
                jump_labels.insert(line.trim().strip_suffix(":").unwrap(), idx);
            } else {
                lines.push((idx, line));
            }
        }

        Self {
            lines,
            jump_labels,
            index: 0,
        }
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Result<Statement<'s>, StatementParseError<'s>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (_, line) = self.lines.get(self.index)?;
        let split: Vec<_> = line.split_whitespace().collect();

        self.index += 1;

        Some(Statement::parse(&split, &self.jump_labels))
    }
}
