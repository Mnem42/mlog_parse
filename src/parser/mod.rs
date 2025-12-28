use std::collections::HashMap;
use std::iter::Filter;
use std::{str::Lines };
use lazy_static::lazy_static;
use regex::Regex;

/// Instructions
pub mod instructions;
/// Error handling
pub mod errs;
pub(self) mod instr_gen;

use instructions::Statement;
use errs::InstructionParseError;

/// A lexer for mindustry logic.
pub struct Lexer<'a> {
    lines: Box<dyn Iterator<Item = &'a str> + 'a>,
    jump_labels: HashMap<&'a str, usize>
}

/// Parse a literal with a prefix (e.g. 0x05) with a given radix.
fn parse_nradix_literal(text: &str, radix: u32) -> i64{
    let mut chars = text.chars();

    match chars.next().unwrap() {
        sign @ ('+' | '-') => {
            let value: String = chars.skip(2).collect();
            i64::from_str_radix(&value, radix).unwrap() * if sign == '-' { -1 } else { 1 }
        },
        _   => i64::from_str_radix(&(chars.skip(1).collect::<String>()), radix).unwrap()
    }
}

lazy_static! {
    static ref JUMPLABEL_REGEX: regex::Regex = Regex::new(r#"^\s*?[[:word:]]+:"#).unwrap();
}

impl<'a> Lexer<'a> {

    /// Create a new lexer
    pub fn new(str: &'a str) -> Self {
        let lines: Filter<Lines<'a>, _> = str
                .lines()
                .filter(|x| x.contains(|x: char| !x.is_whitespace()));


        let mut jump_labels = HashMap::new();
        for (i, x) in lines.clone().enumerate() {
            if let Some(lbl_capture) = (|| JUMPLABEL_REGEX.captures(x)?.get(0))() {
                jump_labels.insert(lbl_capture.as_str(), i);
            }
        }

        println!("{:#?}", lines.clone().collect::<Vec<_>>());

        Self {
            lines: Box::new(lines),
            jump_labels
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Statement, InstructionParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        let split: Vec<_> = line.split_whitespace().collect();
        println!("{:?}", split);

        Some(Statement::parse(&split, &self.jump_labels))
    }
}