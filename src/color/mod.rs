mod cga;
mod hex;
mod palette;
mod rgb;
pub use self::cga::CGA;
pub use self::palette::Palette;
pub use self::rgb::RGB;
use regex::Regex;
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    CustomPalette { front: RGB<u8>, back: RGB<u8> },
    SingleColor(CGA),
    CGA,
    Color,
    BlackAndWhite,
}

impl Mode {
    fn custom_palette_from_cga(cga: CGA) -> Self {
        match cga {
            super::CGA::Black => Mode::CustomPalette {
                front: RGB(0, 0, 0),
                back: RGB(255, 255, 255),
            },
            cga => Mode::CustomPalette {
                front: unsafe { RGB::from_hex(cga.to_hex()) },
                back: RGB(0, 0, 0),
            },
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mode::Color => write!(f, "mode_color"),
            Mode::CGA => write!(f, "mode_cga"),
            Mode::SingleColor(color) => write!(f, "mode_1bit_{}", color),
            Mode::BlackAndWhite => write!(f, "mode_bw"),
            Mode::CustomPalette { front, back } => write!(f, "mode_{:x}_{:x}", front, back),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOption,
    BadPaletteColor(u32),
    CouldNotParsePalette(std::num::ParseIntError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownOption => write!(f, "unknown color option"),
            Error::BadPaletteColor(n) => write!(
                f,
                "palette colors must be between 0 and 0xffffff, but had {:x}",
                n
            ),
            Error::CouldNotParsePalette(err) => write!(f, "could not parse palette: {}", err),
        }
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::CouldNotParsePalette(err)
    }
}
impl FromStr for Mode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PALETTE_RE: Regex = Regex::new("0x([0-9a-fA-F]+) 0x([0-9a-fA-F]+)").unwrap();
        }
        match s.to_lowercase().as_ref() {
            "bw" => Ok(Mode::BlackAndWhite),
            "c" | "color" => Ok(Mode::Color),
            "cga" => Ok(Mode::CGA),
            color if color.parse::<CGA>().is_ok() => {
                Ok(Mode::SingleColor(color.parse::<CGA>().unwrap()))
            }
            palette if PALETTE_RE.is_match(palette) => {
                let caps = PALETTE_RE.captures(palette).unwrap();
                let parse = |cap| match u32::from_str_radix(cap, 16) {
                    Ok(n) if n < 0xff_ff_ff => Ok(unsafe { RGB::from_hex(n) }),
                    Ok(n) => Err(Error::BadPaletteColor(n)),
                    Err(err) => Err(Error::CouldNotParsePalette(err)),
                };
                Ok(Mode::CustomPalette {
                    front: parse(&caps[1])?,
                    back: parse(&caps[2])?,
                })
            }
            _ => Err(Error::UnknownOption),
        }
    }
}

#[test]
fn test_parse() {
    let tt: Vec<(&str, Result<Mode, Error>)> = vec![
        ("bw", Ok(Mode::BlackAndWhite)),
        ("c", Ok(Mode::Color)),
        ("color", Ok(Mode::Color)),
        ("cga", Ok(Mode::CGA)),
        ("WHITE", Ok(Mode::SingleColor(CGA::White))),
        ("LigHT_CYAN", Ok(Mode::SingleColor(CGA::LightCyan))),
        (
            "0x00ffab 0xaa0000",
            Ok(Mode::CustomPalette {
                front: RGB(0, 0xff, 0xab),
                back: RGB(0xaa, 0, 0),
            }),
        ),
        ("alksdalksdsj", Err(Error::UnknownOption)),
        (
            "0x1ffffff 0x123129",
            Err(Error::BadPaletteColor(0x1_ff_ff_ff)),
        ),
    ];
    for (s, want) in tt {
        assert_eq!(s.parse::<Mode>(), want);
    }
}
