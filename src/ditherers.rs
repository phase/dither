//! from Image Dithering: Eleven Algorithms and source code:
//! by Tanner Helland
//! http://www.tannerhelland.com/4660/dithering-eleven-algorithms-source-code/
//! The first – and arguably most famous – 2D error diffusion formula was published by Robert Floyd and Louis Steinberg in 1976. It diffuses errors in the following pattern:
//!
//!
//!
//!       X   7
//!   3   5   1
//!
//!     (1/16)
//!
//! In the notation above, “X” refers to the current pixel. The fraction at the bottom represents the divisor for the error. Said another way, the Floyd-Steinberg formula could be written as:
//!
//!
//!
//!           X    7/16
//!   3/16  5/16   1/16
//!
//!
//!But that notation is long and messy, so I’ll stick with the original.
//!
//!To use our original example of converting a pixel of value “96” to 0 (black) or 255 (white), if we force the pixel to black, the resulting error is 96. We then propagate that error to the surrounding pixels by dividing 96 by 16 ( = 6), then multiplying it by the appropriate values, e.g.:
//!
//!
//!
//!           X     +42
//!   +18    +30    +6
//!
//!
//!By spreading the error to multiple pixels, each with a different value, we minimize any distracting bands of speckles like the original error diffusion example.
use super::Dither;
// Stucki Dithering
///
///             X   8   4
///     2   4   8   4   2
///     1   2   4   2   1
///        (1/42)     
pub struct Stucki;
///
///         X   1   1
///     1   1   1
///         1
///
///       (1/8)
pub struct Atkinson;

///       X   7
///   3   5   1
///
///     (1/16)
pub struct FloydSteinberg;

#[derive(Debug)]
pub struct ErrorUnknownDitherer(String);
impl std::fmt::Display for ErrorUnknownDitherer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unknown ditherer: {}", self.0)
    }
}
impl std::error::Error for ErrorUnknownDitherer {}

impl std::str::FromStr for Ditherer {
    type Err = ErrorUnknownDitherer;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_ref() {
            "floyd" | "steinberg" | "floydsteinberg" | "floyd steinberg" => {
                Ditherer::FloydSteinberg
            }
            "atkinson" => Ditherer::Atkinson,
            "stucki" => Ditherer::Stucki,
            s => return Err(ErrorUnknownDitherer(s.to_string())),
        })
    }
}
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum Ditherer {
    /// [FloydSteinberg]
    FloydSteinberg,
    /// [Atkinson]
    Atkinson,
    /// [Stucki]
    Stucki,
}

impl Dither for Atkinson {
    const DIV: i16 = 8;
    const OFFSETS: &'static [(isize, isize, i16)] = &[
        (1, 0, 1),
        (2, 0, 1),
        //
        (-1, 1, 1),
        (0, 1, 1),
        (1, 1, 1),
        //
        (0, 2, 1),
    ];
}

impl Dither for FloydSteinberg {
    const DIV: i16 = 16;
    const OFFSETS: &'static [(isize, isize, i16)] = &[(1, 0, 7), (-1, 1, 7), (0, 1, 5), (1, 1, 1)];
}
impl Dither for Stucki {
    const DIV: i16 = 42;
    const OFFSETS: &'static [(isize, isize, i16)] = &[
        (1, 0, 8),
        (2, 0, 4),
        (-2, 1, 2),
        (-1, 1, 4),
        //
        (0, 1, 8),
        (1, 1, 4),
        (2, 1, 2),
        (-2, 2, 1),
        (-1, 2, 2),
        //
        (0, 2, 4),
        (1, 2, 2),
        (2, 2, 1),
    ];
}
