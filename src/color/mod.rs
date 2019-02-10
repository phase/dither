mod cga;
mod hex;
mod palette;
mod rgb;
pub use self::cga::CGA;
pub use self::palette::{Palette, PaletteError};
pub use self::rgb::RGB;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    CustomPalette(Palette),
    SingleColor(CGA),
    CGA,
    Color,
    BlackAndWhite,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mode::Color => write!(f, "mode_color"),
            Mode::CGA => write!(f, "mode_cga"),
            Mode::SingleColor(color) => write!(f, "mode_1bit_{}", color),
            Mode::BlackAndWhite => write!(f, "mode_bw"),
            Mode::CustomPalette(palette) => write!(f, "mode_{:x}", palette),
        }
    }
}

impl std::str::FromStr for Mode {
    type Err = PaletteError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_ref() {
            "bw" => Mode::BlackAndWhite,
            "c" | "color" => Mode::Color,
            "cga" => Mode::CGA,
            s => {
                if let Ok(color) = s.parse::<CGA>() {
                    Mode::SingleColor(color)
                } else {
                    Mode::CustomPalette(s.parse::<Palette>()?)
                }
            }
        })
    }
}
