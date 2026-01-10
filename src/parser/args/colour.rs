use std::{
    collections::HashMap,
    fmt::{self, Display, Write},
    num::ParseIntError,
    str::FromStr,
    sync::LazyLock,
};
use thiserror::Error;

#[cfg(feature = "rgb_crate")]
use crate::parser;

#[rustfmt::skip]
static COLOURS: LazyLock<HashMap<&str, Rgba>> = LazyLock::new(|| {
    HashMap::from([
        ("white",     Rgba { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF }),
        ("lightGray", Rgba { r: 0xBF, g: 0xBF, b: 0xBF, a: 0xFF }),
        ("gray",      Rgba { r: 0x7F, g: 0x7F, b: 0x7F, a: 0xFF }),
        ("darkGray",  Rgba { r: 0x3F, g: 0x3F, b: 0x3F, a: 0xFF }),
        ("black",     Rgba { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }),
        ("clear",     Rgba { r: 0x00, g: 0x00, b: 0x00, a: 0x00 }),

        ("red",       Rgba { r: 0xE5, g: 0x54, b: 0x54, a: 0xFF }),
        ("scarlet",   Rgba { r: 0xFF, g: 0x34, b: 0x1C, a: 0xFF }),
        ("crimson",   Rgba { r: 0xDC, g: 0x14, b: 0x3C, a: 0xFF }),
        ("coral",     Rgba { r: 0xFF, g: 0x7F, b: 0x50, a: 0xFF }),
        ("salmon",    Rgba { r: 0xFA, g: 0x80, b: 0x72, a: 0xFF }),
        ("pink",      Rgba { r: 0xFF, g: 0x69, b: 0xB4, a: 0xFF }),
        ("magenta",   Rgba { r: 0xFF, g: 0x00, b: 0xFF, a: 0xFF }),

        ("yellow",    Rgba { r: 0xFF, g: 0xFF, b: 0x00, a: 0xFF }),
        ("gold",      Rgba { r: 0xFF, g: 0xD7, b: 0x00, a: 0xFF }),
        ("goldenrod", Rgba { r: 0xDA, g: 0xA5, b: 0x20, a: 0xFF }),
        ("orange",    Rgba { r: 0xFF, g: 0xA5, b: 0x00, a: 0xFF }),

        ("brown",     Rgba { r: 0x8B, g: 0x45, b: 0x13, a: 0xFF }),
        ("tan",       Rgba { r: 0xD2, g: 0xB4, b: 0x8C, a: 0xFF }),
        ("brick",     Rgba { r: 0xB2, g: 0x22, b: 0x22, a: 0xFF }),
   
        ("green",     Rgba { r: 0x3D, g: 0xD6, b: 0x67, a: 0xFF }),
        ("acid",      Rgba { r: 0x7F, g: 0xFF, b: 0x00, a: 0xFF }),
        ("lime",      Rgba { r: 0x32, g: 0xCD, b: 0x32, a: 0xFF }),
        ("forest",    Rgba { r: 0x22, g: 0x8B, b: 0x22, a: 0xFF }),
        ("olive",     Rgba { r: 0x6B, g: 0x8E, b: 0x23, a: 0xFF }),
   
        ("blue",      Rgba { r: 0x00, g: 0x00, b: 0xFF, a: 0xFF }),
        ("navy",      Rgba { r: 0x00, g: 0x00, b: 0x7F, a: 0xFF }),
        ("royal",     Rgba { r: 0x41, g: 0x69, b: 0xE1, a: 0xFF }),
        ("slate",     Rgba { r: 0x70, g: 0x80, b: 0x90, a: 0xFF }),
        ("sky",       Rgba { r: 0x87, g: 0xCE, b: 0xEB, a: 0xFF }),
        ("cyan",      Rgba { r: 0x00, g: 0xFF, b: 0xFF, a: 0xFF }),
        ("teal",      Rgba { r: 0x00, g: 0x7F, b: 0x7F, a: 0xFF }),
   
        ("purple",    Rgba { r: 0x02, g: 0x0F, b: 0x0F, a: 0xFF }),
        ("violet",    Rgba { r: 0xEE, g: 0x82, b: 0xEE, a: 0xFF }),
        ("maroon",    Rgba { r: 0xB0, g: 0x30, b: 0x60, a: 0xFF }),
    ])
});

// Only really relevant for external use, it's internally parsed with separate functions that assume
// the input is valid

/// An error from parsing a colour, created by the [`FromStr`] implementation
#[derive(Debug, Error, PartialEq)]
pub enum ColourParseError {
    /// An invalid named colour
    #[error("The colour name {0} is invalid")]
    InvalidColourName(String),

    /// An invalid number
    #[error("Error {0} returned when parsing int")]
    InvalidInt(#[from] ParseIntError),

    /// An invalid colour literal
    #[error("The string {0} is an invalid colour literal")]
    InvalidColourLiteral(String),
}

/// A colour. Use this instead of converting to [`rgb::RGBA8`] if you need to print it out in the
/// format mindustry uses or need to parse from it.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8,
}

impl Default for Rgba {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { r, g, b, a } = self;
        f.write_char('%')?;
        for color in [r, g, b, a] {
            write!(f, "{color:0>2x}")?;
        }
        Ok(())
    }
}

// Mostly for external use
impl FromStr for Rgba {
    type Err = ColourParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // %[name] syntax
        if s.starts_with("%[") && s.ends_with("]") {
            let colour_name = &s[2..s.len() - 1].to_lowercase();

            if let Some(colour) = COLOURS.get(colour_name.as_str()) {
                Ok(*colour)
            } else {
                Err(ColourParseError::InvalidColourName(colour_name.to_string()))
            }
        } else if s.starts_with("%") {
            if (s.len() == 7 || s.len() == 9) && s.chars().skip(1).all(|x| x.is_ascii_hexdigit()) {
                Ok(Rgba::from_hex_literal_unchecked(s))
            } else {
                Err(ColourParseError::InvalidColourLiteral(s.to_string()))
            }
        } else {
            Err(ColourParseError::InvalidColourLiteral(s.to_string()))
        }
    }
}

impl Rgba {
    /// Parses a colour literal from the format %[name]. Only use this if the literal has already
    /// been checked (e.g. from a regex).
    ///
    /// Returns [`Some`] if the named colour exists, and [`None`] otherwise.
    ///
    /// # Panics
    /// On any problem, at all.
    pub(super) fn from_named_literal_unchecked(literal: &str) -> Option<Self> {
        let colour_name = &literal[2..literal.len() - 1].to_lowercase();
        COLOURS.get(colour_name.as_str()).copied()
    }

    /// Parses a colour literal in the format %RRGGBB or %RRGGBBAA. Only use this if the literal
    /// has already been checked (e.g. from a regex).
    ///
    /// # Panics
    /// On any problem, at all.
    pub(super) fn from_hex_literal_unchecked(literal: &str) -> Self {
        let mut channels = (1..literal.len())
            .step_by(2)
            .map(|start| literal.get(start..start + 2));
        let mut get_channel = || {
            let Some(Some(channel)) = channels.next() else {
                return u8::from_str_radix("", 16);
            };

            Ok(i16::from_str_radix(channel, 16).unwrap() as u8)
        };
        let mut color = Self::default();
        let Self { r, g, b, a } = &mut color;

        for channel in [r, g, b] {
            *channel = get_channel().unwrap();
        }

        if literal.len() == 9 {
            *a = get_channel().unwrap()
        }

        color
    }
}

#[cfg(feature = "rgb_crate")]
impl From<Rgba> for rgb::RGBA8 {
    fn from(v: parser::args::colour::Rgba) -> rgb::RGBA8 {
        rgb::Rgba {
            r: v.r,
            g: v.g,
            b: v.b,
            a: v.a,
        }
    }
}
