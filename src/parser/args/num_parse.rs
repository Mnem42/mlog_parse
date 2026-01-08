//! General functions to parse numbers

/// Parses a literal with a prefix (e.g. 0x05) with a given radix..
pub(super) fn parse_nradix_literal(text: &str, radix: u32) -> i64 {
    let mut chars = text.chars();

    match chars.next().unwrap() {
        sign @ ('+' | '-') => {
            i64::from_str_radix(&text[3..], radix).unwrap() * if sign == '-' { -1 } else { 1 }
        }
        _ => i64::from_str_radix(&text[2..], radix).unwrap(),
    }
}