use super::RGB;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CGA {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    Gray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

impl CGA {
    pub const COLORS: [CGA; 16] = [
        CGA::Black,
        CGA::Blue,
        CGA::Green,
        CGA::Cyan,
        CGA::Red,
        CGA::Magenta,
        CGA::Brown,
        CGA::LightCyan,
        CGA::Gray,
        CGA::LightBlue,
        CGA::LightGreen,
        CGA::LightCyan,
        CGA::LightRed,
        CGA::LightMagenta,
        CGA::Yellow,
        CGA::White,
    ];
    pub fn to_hex(self) -> u32 {
        match self {
            CGA::Black => hex::BLACK,
            CGA::Blue => hex::BLUE,
            CGA::Green => hex::GREEN,
            CGA::Cyan => hex::CYAN,
            CGA::Red => hex::RED,
            CGA::Magenta => hex::MAGENTA,
            CGA::Brown => hex::BROWN,
            CGA::LightGray => hex::LIGHT_GRAY,
            CGA::Gray => hex::GRAY,
            CGA::LightBlue => hex::LIGHT_BLUE,
            CGA::LightGreen => hex::LIGHT_GREEN,
            CGA::LightCyan => hex::LIGHT_CYAN,
            CGA::LightRed => hex::LIGHT_RED,
            CGA::LightMagenta => hex::LIGHT_MAGENTA,
            CGA::Yellow => hex::YELLOW,
            CGA::White => hex::WHITE,
        }
    }
    /// quantize a RGB triplet to the closest CGA color and error.
    /// this is a somewhat naive/ inefficient implementation; i'm sure I can do better.
    ///
    /// ```
    /// # use dither::cga::quantize;
    /// # use dither::CGA;
    /// # use dither::RGB;
    /// let offset = RGB(1.0, -1.0, 1.0);
    /// // note all are off by one; ok to test absolute equality since they're such small floats
    /// assert_eq!(
    ///     (RGB::from(CGA::cyan), -offset),
    ///     quantize(RGB::from(CGA::Cyan)+offset)
    /// );
    /// ```
    pub fn quantize(RGB(r0, g0, b0): RGB<f64>) -> (RGB<f64>, RGB<f64>) {
        let mut min_abs_err = std::f64::INFINITY;
        let mut closest = RGB::default();
        let mut min_err = RGB::default();

        for RGB(r1, g1, b1) in CGA::COLORS.iter().cloned().map(RGB::<f64>::from) {
            let abs_err = f64::abs(r0 - r1) + f64::abs(g0 - g1) + f64::abs(b0 - b1);
            if abs_err < min_abs_err {
                min_err = RGB(r0 - r1, g0 - g1, b0 - b1);
                closest = RGB(r1, g1, b1);
                min_abs_err = abs_err;
            }
        }
        (closest, min_err)
    }

    pub fn try_from_hex(hex: u32) -> Option<Self> {
        Some(match hex {
            hex::BLACK => CGA::Black,
            hex::BLUE => CGA::Blue,
            hex::GREEN => CGA::Green,
            hex::CYAN => CGA::Cyan,
            hex::RED => CGA::Red,
            hex::MAGENTA => CGA::Magenta,
            hex::BROWN => CGA::Brown,
            hex::LIGHT_GRAY => CGA::LightGray,
            hex::GRAY => CGA::Gray,
            hex::LIGHT_BLUE => CGA::LightBlue,
            hex::LIGHT_GREEN => CGA::LightGreen,
            hex::LIGHT_CYAN => CGA::LightCyan,
            hex::LIGHT_RED => CGA::LightRed,
            hex::LIGHT_MAGENTA => CGA::LightMagenta,
            hex::YELLOW => CGA::Yellow,
            hex::WHITE => CGA::White,
            _ => return None,
        })
    }
}

pub mod hex {
    pub const BLACK: u32 = 0x00_00_00;
    pub const BLUE: u32 = 0x00_00_AA;
    pub const GREEN: u32 = 0x00_AA_00;
    pub const CYAN: u32 = 0x00_AA_AA;
    pub const RED: u32 = 0xAA_00_00;
    pub const MAGENTA: u32 = 0xAA_00_AA;
    pub const BROWN: u32 = 0xAA_55_00;
    pub const LIGHT_GRAY: u32 = 0xAA_AA_AA;
    pub const GRAY: u32 = 0x55_55_55;
    pub const LIGHT_BLUE: u32 = 0x55_55_FF;
    pub const LIGHT_GREEN: u32 = 0x55_FF_55;
    pub const LIGHT_CYAN: u32 = 0x55_FF_FF;
    pub const LIGHT_RED: u32 = 0xFF_55_55;
    pub const LIGHT_MAGENTA: u32 = 0xFF_55_FF;
    pub const YELLOW: u32 = 0xFF_FF_55;
    pub const WHITE: u32 = 0xFF_FF_FF;

}
