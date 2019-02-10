use super::hex;
use super::RGB;
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub struct Palette {
    pub front: RGB<u8>,
    pub back: RGB<u8>,
}

use std::fmt::{Formatter, LowerHex};
impl LowerHex for Palette {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:x}_{:x}", self.front, self.back)
    }
}

// --- CONVERSIONS  ---

impl From<super::CGA> for Palette {
    fn from(cga: super::CGA) -> Self {
        match cga {
            super::CGA::Black => Palette {
                front: RGB(0, 0, 0),
                back: RGB(255, 255, 255),
            },
            cga => Palette {
                front: unsafe { RGB::from_hex(cga.to_hex()) },
                back: RGB(0, 0, 0),
            },
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
/// A PaletteError is an error parsing a palette from user input.
pub enum PaletteError {
    UnknownColor(u32),
    CouldNotParse(std::num::ParseIntError),
    UnknownOption,
}

impl std::fmt::Display for PaletteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PaletteError::UnknownColor(n) => write!(
                f,
                "unknown color {:x} must be between 0x00 and 0xFF_FF_FF",
                n
            ),
            PaletteError::CouldNotParse(err) => err.fmt(f),
            PaletteError::UnknownOption => write!(
                f,
                r#"unknown color mode. available modes are "color", "bw", "cga", "light_green", "cyan", 
                and a user-specified palette as a pair of hexidecimal numbers: i.e "0xff0000 0x00aa00" "#
            ),
        }
    }
}

impl std::error::Error for PaletteError {}
impl std::str::FromStr for Palette {
    type Err = PaletteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse = |s| match u32::from_str_radix(s, 16) {
            Err(err) => Err(PaletteError::CouldNotParse(err)),
            Ok(n) if n > 0x_FF_FF_FF => Err(PaletteError::UnknownColor(n)),
            Ok(n) => Ok(unsafe { RGB::from_hex(n) }),
        };
        let mut it = s.split_whitespace();
        if let (Some(front), Some(back)) = (it.next(), it.next()) {
            Ok(Palette {
                front: parse(front)?,
                back: parse(back)?,
            })
        } else {
            Err(PaletteError::UnknownOption)
        }
    }
}
