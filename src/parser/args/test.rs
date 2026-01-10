use std::str::FromStr;
use crate::parser::args::{Argument, Rgba, colour::ColourParseError};

#[test]
fn colour_literal_unchecked() {
    assert_eq!(
        Rgba::from_hex_literal_unchecked("%ffaaffaa"),
        Rgba {
            r: 0xFF,
            g: 0xAA,
            b: 0xFF,
            a: 0xAA
        }
    )
}

#[test]
fn colour_named_literal_unchecked() {
    assert_eq!(
        Rgba::from_named_literal_unchecked("%[rEd]"),
        Some(Rgba {
            r: 0xE5,
            g: 0x54,
            b: 0x54,
            a: 0xFF
        })
    );

    assert_eq!(Rgba::from_named_literal_unchecked("%[foo]"), None);
}

#[test]
fn colour_fromstr() {
    assert_eq!(
        Rgba::from_str("%012345"),
        Ok(Rgba {
            r: 0x01,
            g: 0x23,
            b: 0x45,
            a: 0xFF
        })
    );
    assert_eq!(
        Rgba::from_str("%01234567"),
        Ok(Rgba {
            r: 0x01,
            g: 0x23,
            b: 0x45,
            a: 0x67
        })
    );

    assert_eq!(
        Rgba::from_str("%[rED]"),
        Ok(Rgba {
            r: 0xE5,
            g: 0x54,
            b: 0x54,
            a: 0xFF
        })
    );
    assert_eq!(
        Rgba::from_str("%[notacolour]"),
        Err(ColourParseError::InvalidColourName(
            "notacolour".to_string()
        ))
    );

    assert_eq!(
        Rgba::from_str("%01234"),
        Err(ColourParseError::InvalidColourLiteral("%01234".to_string()))
    );
    assert_eq!(
        Rgba::from_str("%0123456"),
        Err(ColourParseError::InvalidColourLiteral(
            "%0123456".to_string()
        ))
    );
    assert_eq!(
        Rgba::from_str("%notacolour"),
        Err(ColourParseError::InvalidColourLiteral(
            "%notacolour".to_string()
        ))
    );
}

#[test]
fn strange_args() {
    // Why would you ever want this
    assert_eq!(
        Argument::from("--1.+2"),
        Argument::Number(1.02)
    )
}

#[test]
fn strange_colour_literals() {
    assert_eq!(
        Rgba::from_hex_literal_unchecked("%-f-0-1-A"),
        Rgba {
            r: 241,
            g: 0,
            b: 255,
            a: 246
        }
    )
}

#[test]
fn display() {
    assert_eq!(
        Argument::GlobalVar("counter").to_string(),
        "@counter".to_string()
    );

    assert_eq!(Argument::Number(12.345).to_string(), "12.345".to_string());

    assert_eq!(Argument::String("test").to_string(), "\"test\"".to_string());
    assert_eq!(Argument::Variable("thing").to_string(), "thing".to_string());

    assert_eq!(
        Argument::Colour(Rgba {
            r: 0xFF,
            g: 0xFF,
            b: 0xFF,
            a: 0xFF
        })
        .to_string(),
        "%ffffffff".to_string()
    )
}
