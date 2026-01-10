use std::{
    fmt::{self, Display, Write},
    num::ParseIntError,
    str::FromStr,
};
#[cfg(feature = "rgb_crate")]
use rgb::RGBA8;
use thiserror::Error;

#[cfg(feature = "rgb_crate")]
use crate::parser;
use crate::parser::args::named_colours::NAMED_COLOURS;

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

            if let Some(colour) = NAMED_COLOURS.get(colour_name.as_str()) {
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
        NAMED_COLOURS.get(colour_name.as_str()).copied()
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

#[cfg(feature = "rgb_crate")]
impl From<RGBA8> for Rgba {
    fn from(v: RGBA8) -> Self {
        Rgba {
            r: v.r,
            g: v.g,
            b: v.b,
            a: v.a
        }
    }
}