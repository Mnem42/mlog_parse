use std::str::FromStr;

use crate::parser::args::{Rgba, colour::ColourParseError};

#[test]
fn colour_literal_unchecked() {
    assert_eq!(
        Rgba::from_hex_literal_unchecked("%ffaaffaa"),
        Rgba { r: 0xFF, g: 0xAA, b: 0xFF, a: 0xAA }
    )
}

#[test]
fn colour_named_literal_unchecked() {
    assert_eq!(
        Rgba::from_named_literal_unchecked("%[rEd]"),
        Some(Rgba { r: 0xE5, g: 0x54, b: 0x54, a: 0xFF })
    );

    assert_eq!(
        Rgba::from_named_literal_unchecked("%[foo]"),
        None
    );
}

#[test]
fn colour_fromstr() {
    assert_eq!(
        Rgba::from_str("%012345"),
        Ok(Rgba { r: 0x01, g: 0x23, b: 0x45, a: 0xFF})
    );
    assert_eq!(
        Rgba::from_str("%01234567"),
        Ok(Rgba { r: 0x01, g: 0x23, b: 0x45, a: 0x67})
    );

    assert_eq!(
        Rgba::from_str("%[rED]"),
        Ok(Rgba { r: 0xE5, g: 0x54, b: 0x54, a: 0xFF })
    );
    assert_eq!(
        Rgba::from_str("%[notacolour]"),
        Err(ColourParseError::InvalidColourName("notacolour".to_string()))
    );

    assert_eq!(
        Rgba::from_str("%01234"),
        Err(ColourParseError::InvalidColourLiteral("%01234".to_string()))
    );
    assert_eq!(
        Rgba::from_str("%0123456"),
        Err(ColourParseError::InvalidColourLiteral("%0123456".to_string()))
    );
    assert_eq!(
        Rgba::from_str("%notacolour"),
        Err(ColourParseError::InvalidColourLiteral("%notacolour".to_string()))
    );
}