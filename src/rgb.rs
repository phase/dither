use std::ops::{Add, Div, Mul};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct RGB<N>(pub N, pub N, pub N);

impl RGB<f64> {
    pub fn to_chroma_corrected_black_and_white(&self) -> f64 {
        let RGB(r, g, b) = self;
        r * 0.2126 + g * 0.7152 + b * 0.0722
    }
}

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
            (RGB(r_quot, g_quot, b_quot), RGB(r_rem, g_rem, b_rem))
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
impl RGB<u8> {
    pub fn from_chroma_corrected_black_and_white(p: f64) -> Self {
        RGB(clamp_f64_to_u8(p), clamp_f64_to_u8(p), clamp_f64_to_u8(p))
    }
}

// ---- OPERATOR OVERLOADING ---- //

impl<N> Add for RGB<N>
where
    N: Add<Output = N>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let RGB(r0, g0, b0) = self;
        let RGB(r1, g1, b1) = other;
        RGB(r0 + r1, g0 + g1, b0 + b1)
    }
}

impl<N> Div for RGB<N>
where
    N: Div<Output = N>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let RGB(r0, g0, b0) = self;
        let RGB(r1, g1, b1) = other;
        RGB(r0 / r1, g0 / g1, b0 / b1)
    }
}

impl<N> Mul for RGB<N>
where
    N: Mul<Output = N>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let RGB(r0, g0, b0) = self;
        let RGB(r1, g1, b1) = other;
        RGB(r0 * r1, g0 * g1, b0 * b1)
    }
}

// scalar multiplication
impl<S: Mul<Output = S> + Copy> Mul<S> for RGB<S> {
    type Output = Self;
    fn mul(self, s: S) -> Self {
        let RGB(r, g, b) = self;
        RGB(r * s, g * s, b * s)
    }
}

// scalar division
impl<S: Div<Output = S> + Copy> Div<S> for RGB<S> {
    type Output = Self;
    fn div(self, s: S) -> Self {
        let RGB(r, g, b) = self;
        RGB(r / s, g / s, b / s)
    }
}

// --- CONVERSIONS  ---

impl From<RGB<u8>> for RGB<i16> {
    fn from(RGB(r, g, b): RGB<u8>) -> Self {
        RGB(i16::from(r), i16::from(g), i16::from(b))
    }
}
impl From<RGB<u8>> for RGB<f64> {
    fn from(RGB(r, g, b): RGB<u8>) -> Self {
        RGB(f64::from(r), f64::from(g), f64::from(b))
    }
}

impl From<RGB<i16>> for RGB<f64> {
    fn from(RGB(r, g, b): RGB<i16>) -> Self {
        RGB(f64::from(r), f64::from(g), f64::from(b))
    }
}

impl From<RGB<i16>> for RGB<u8> {
    fn from(RGB(r, g, b): RGB<i16>) -> Self {
        RGB(r as u8, g as u8, b as u8)
    }
}
