use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct RGB<N>(pub N, pub N, pub N);

impl Copy for RGB<u8> {}

impl<P> RGB<P> {
    pub fn convert_with<Q>(self, mut convert: impl FnMut(P) -> Q) -> RGB<Q> {
        let RGB(r, g, b) = self;
        RGB(convert(r), convert(g), convert(b))
    }
    /// this higher-order function takes a function from function from P to (P, P)
    /// and creates the equivalent function that maps it across RGB<P>.
    pub fn map_across(
        mut quantize: impl FnMut(P) -> (P, P),
    ) -> impl FnMut(RGB<P>) -> (RGB<P>, RGB<P>) {
        move |RGB(r, g, b)| {
            let (r_quot, r_rem) = quantize(r);
            let (g_quot, g_rem) = quantize(g);
            let (b_quot, b_rem) = quantize(b);
            let quotient = RGB(r_quot, g_quot, b_quot);
            let remainder = RGB(r_rem, g_rem, b_rem);
            (quotient, remainder)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub struct Palette {
    pub front: RGB<u8>,
    pub back: RGB<u8>,
}

#[derive(Debug)]
pub enum PaletteError {
    UnknownColor(u32),
    CouldNotParse(std::num::ParseIntError),
    NotPair,
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
            PaletteError::NotPair => write!(f, "could not parse palette input; need to supply a pair of hexadecimal numbers between 0x00 and 0xFF_FF_FF, eg, \"0xff0000 0x00aa_00\"")
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
            Ok(n) => Ok(RGB::from_hex(n)),
        };
        let mut it = s.split_whitespace();
        if let (Some(front), Some(back)) = (it.next(), it.next()) {
            Ok(Palette {
                front: parse(front)?,
                back: parse(back)?,
            })
        } else {
            Err(PaletteError::NotPair)
        }
    }
}

pub fn clamp_f64_to_u8(n: f64) -> u8 {
    match n {
        n if n > 255.0 => 255,
        n if n < 0.0 => 0,
        n => n as u8,
    }
}

// ---- OPERATOR OVERLOADING ---- //

// binary vec additon, subtraction

impl<N: Add<Output = N>> Add for RGB<N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let RGB(r0, g0, b0) = self;
        let RGB(r1, g1, b1) = other;
        RGB(r0 + r1, g0 + g1, b0 + b1)
    }
}

impl<N: Sub<Output = N>> Sub for RGB<N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let RGB(r0, g0, b0) = self;
        let RGB(r1, g1, b1) = other;
        RGB(r0 - r1, g0 - g1, b0 - b1)
    }
}

// scalar ops for RGB<N> and N
impl<S: Mul<Output = S> + Copy> Mul<S> for RGB<S> {
    type Output = Self;
    fn mul(self, s: S) -> Self {
        self.convert_with(|c| c * s)
    }
}

impl<S: Div<Output = S> + Copy> Div<S> for RGB<S> {
    type Output = Self;
    fn div(self, s: S) -> Self {
        self.convert_with(|c| c / s)
    }
}

impl<S: Rem<Output = S> + Copy> Rem<S> for RGB<S> where {
    type Output = Self;
    fn rem(self, s: S) -> Self {
        self.convert_with(|c| c % s)
    }
}

// unary ops

impl<N: Neg<Output = N>> Neg for RGB<N> where {
    type Output = Self;
    fn neg(self) -> Self {
        self.convert_with(|c| -c)
    }
}

impl From<super::CGA> for RGB<u8> {
    fn from(cga: super::CGA) -> Self {
        Self::from_hex(cga.to_hex())
    }
}

impl From<super::CGA> for RGB<f64> {
    fn from(cga: super::CGA) -> Self {
        Self::from(RGB::<u8>::from(cga))
    }
}
impl From<RGB<u8>> for RGB<f64> {
    fn from(rgb: RGB<u8>) -> Self {
        rgb.convert_with(f64::from)
    }
}

impl<N, M> From<[N; 3]> for RGB<M>
where
    N: Copy,
    M: From<N>,
{
    fn from(a: [N; 3]) -> Self {
        RGB(a[0], a[1], a[2]).convert_with(M::from)
    }
}

impl<N, M> From<(N, N, N)> for RGB<M>
where
    M: From<N>,
{
    fn from((r, g, b): (N, N, N)) -> Self {
        RGB(r, g, b).convert_with(M::from)
    }
}

impl RGB<f64> {
    pub fn to_chroma_corrected_black_and_white(&self) -> f64 {
        let RGB(r, g, b) = self;
        r * 0.2126 + g * 0.7152 + b * 0.0722
    }
}

impl RGB<u8> {
    pub fn from_chroma_corrected_black_and_white(p: f64) -> Self {
        RGB(clamp_f64_to_u8(p), clamp_f64_to_u8(p), clamp_f64_to_u8(p))
    }

    // --- CONVERSIONS  ---
    const fn from_hex(hex: u32) -> Self {
        RGB((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
    }
}
