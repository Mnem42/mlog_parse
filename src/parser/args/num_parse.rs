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

/// Arc parses hex values in a mildly cursed way for colour literals (which this replicates)
pub(super) fn parse_hex_arcoid(text: &str) -> i64 {
    let mut total = 0i64;

    for (i, char) in text.chars().enumerate() {
        total += i64::from_str_radix(&char.to_string(), 16).unwrap_or(-1)
            * if i == 0 { 16 } else { 1 }
    }

    total
}