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

use super::Img;
use std::ops::{Add, Div, Mul};

/// A type of Dither. Available dithers are [Stucki], [Atkinson], [FloydSteinberg], [Burkes]
pub trait Dither {
    const DIV: i16;
    const OFFSETS: &'static [(isize, isize, i16)];
    /// dither a 2d matrix.
    /// `P` is the type of pixel ([`u8`], [RGB<f64, f64, f64>]);
    /// S is multiplible and divisble by a **S**CALAR
    /// but adds to ITSELF
    ///
    fn dither<P>(img: Img<P>, mut quantize: impl FnMut(P) -> (P, P)) -> super::Img<P>
    where
        P: Add<Output = P> + Mul<i16, Output = P> + Div<i16, Output = P> + Copy + Default,
    {
        let (width, height) = img.size();

        let mut output: Vec<P> = vec![P::default(); img.len()];
        let mut spillover: Vec<P> = vec![P::default(); img.len()];

        for y in 0..height {
            for x in 0..width {
                let i = (y * width + x) as usize;

                let (quantized, spill) = quantize(img[(x, y)] + spillover[i]);
                output[i] = quantized;

                // add spillover matrices
                let (x, y) = (x as isize, y as isize);
                for (dx, dy, mul) in Self::OFFSETS.iter().cloned() {
                    let i = (((y + dy) * width as isize) + (x + dx)) as usize;

                    if let Some(stored_spill) = spillover.get_mut(i) {
                        *stored_spill = *stored_spill + (spill * mul) / Self::DIV;
                    }
                }
            }
        }
        Img { buf: output, width }
    }
}

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

///
///             X   8   4
///     2   4   8   4   2
///
///           (1/32)
pub struct Burkes;

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
            "burkes" => Ditherer::Burkes,
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
    /// [Burkes]
    Burkes,
}

impl Ditherer {
    pub fn dither<P>(self, img: Img<P>, quantize: impl FnMut(P) -> (P, P)) -> Img<P>
    where
        P: Add<Output = P> + Mul<i16, Output = P> + Div<i16, Output = P> + Copy + Default,
    {
        match self {
            Ditherer::FloydSteinberg => FloydSteinberg::dither(img, quantize),
            Ditherer::Atkinson => Atkinson::dither(img, quantize),
            Ditherer::Burkes => Burkes::dither(img, quantize),
            Ditherer::Stucki => Stucki::dither(img, quantize),
        }
    }
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

impl Dither for Burkes {
    const DIV: i16 = 32;
    const OFFSETS: &'static [(isize, isize, i16)] = &[
        (1, 0, 8),
        (2, 0, 4),
        (-2, 1, 2),
        (-1, 1, 4),
        (0, 1, 8),
        (1, 1, 4),
        (2, 1, 2),
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
