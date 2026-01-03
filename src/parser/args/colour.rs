use std::{fmt::{self, Display, Write}, num::{IntErrorKind, ParseIntError}, str::FromStr};



/// A colour
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

impl FromStr for Rgba {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // %[name] syntax


        // %RRGGBBAA or %RRGGBB syntax
        let mut channels = (0..s.len()).step_by(2).map(|start| s.get(start..start + 2));
        let mut get_channel = || {
            let Some(Some(channel)) = channels.next() else {
                return u8::from_str_radix("", 16);
            };
            u8::from_str_radix(channel, 16)
        };
        let mut color = Self::default();
        let Self { r, g, b, a } = &mut color;
        for channel in [r, g, b] {
            *channel = get_channel()?;
        }
        match get_channel() {
            Err(err) if *err.kind() == IntErrorKind::Empty => {}
            Err(err) => return Err(err),
            Ok(alpha) => *a = alpha,
        }
        Ok(color)
    }
}