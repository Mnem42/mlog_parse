use crate::parser::args::Rgba;

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