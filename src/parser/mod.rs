use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Error handling
pub mod errs;
pub(self) mod instr_gen;
/// Instructions
pub mod instructions;

use errs::StatementParseError;
use instructions::Statement;

/// A lexer for mindustry logic.
pub struct Lexer<'a> {
    lines: Vec<(&'a str, usize)>,
    jump_labels: HashMap<&'a str, usize>,
    index: usize,
}

/// Parse a literal with a prefix (e.g. 0x05) with a given radix.
fn parse_nradix_literal(text: &str, radix: u32) -> i64 {
    let mut chars = text.chars();

    match chars.next().unwrap() {
        sign @ ('+' | '-') => {
            let value: String = chars.skip(2).collect();
            i64::from_str_radix(&value, radix).unwrap() * if sign == '-' { -1 } else { 1 }
        }
        _ => i64::from_str_radix(&(chars.skip(1).collect::<String>()), radix).unwrap(),
    }
}

lazy_static! {
    static ref JUMPLABEL_REGEX: regex::Regex = Regex::new(r#"^\s*?[[:word:]]+:"#).unwrap();
}

impl<'a> Lexer<'a> {
    /// Create a new lexer
    pub fn new(str: &'a str) -> Self {
        let (jumps, lines): (Vec<_>, Vec<_>) = str
            .lines()
            .enumerate()
            .map(|(i, x)| (x, i))
            .filter(|(x, _)| x.contains(|x: char| !x.is_whitespace()))
            .partition(|(x, _)| JUMPLABEL_REGEX.is_match(x));

        let jump_labels = HashMap::from_iter(
            jumps
                .into_iter()
                .map(|(x, i)| (x.trim().strip_suffix(":").unwrap(), i)),
        );

        Self {
            lines,
            jump_labels,
            index: 0,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Statement, StatementParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let (line, _) = self.lines.get(self.index)?;
        let split: Vec<_> = line.split_whitespace().collect();

        self.index += 1;

        Some(Statement::parse(&split, &self.jump_labels))
    }
}
