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

impl std::str::FromStr for Palette {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse = |s: &str| match u32::from_str_radix(&s[2..], 16) {
            Err(err) => Err(super::Error::CouldNotParsePalette(err)),
            Ok(n) if n > 0x_FF_FF_FF => Err(super::Error::BadPaletteColor(n)),
            Ok(n) => Ok(unsafe { RGB::from_hex(n) }),
        };
        let mut it = s.split_whitespace();
        if let (Some(front), Some(back)) = (it.next(), it.next()) {
            dbg!(front);
            dbg!(back);
            Ok(Palette {
                front: parse(front)?,
                back: parse(back)?,
            })
        } else {
            Err(super::Error::UnknownOption)
        }
    }
}
