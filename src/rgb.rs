use std::ops::{Add, Div, Mul};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RGB<N>(pub N, pub N, pub N);

const RED: f64 = 0.2126;
const GREEN: f64 = 0.7152;
const BLUE: f64 = 0.0722;
impl<N: Into<f64>> RGB<N> {
    pub fn to_chroma_corrected_black_and_white(RGB(r, g, b): Self) -> u8 {
        clamp_to_u8(r.into() * RED + g.into() * GREEN + b.into() * BLUE)
    }
}

fn clamp_to_u8(n: f64) -> u8 {
    match n {
        n if n > 255.0 => 255,
        n if n < 0.0 => 0,
        n => n as u8,
    }
}
impl RGB<u8> {
    pub fn from_color_corrected_black_and_white(p: i16) -> Self {
        fn clamp(p: i16) -> u8 {
            match p {
                p if p > 255 => 255,
                p if p < 0 => 0,
                p => p as u8,
            }
        }
        RGB(clamp(p), clamp(p), clamp(p))
    }
}

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

impl From<RGB<u8>> for RGB<i16> {
    fn from(RGB(r, g, b): RGB<u8>) -> Self {
        RGB(i16::from(r), i16::from(g), i16::from(b))
    }
}

impl From<RGB<i16>> for RGB<u8> {
    fn from(RGB(r, g, b): RGB<i16>) -> Self {
        RGB(r as u8, g as u8, b as u8)
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

/// scalar division
impl<S: Div<Output = S> + Copy> Div<S> for RGB<S> {
    type Output = Self;
    fn div(self, s: S) -> Self {
        let RGB(r, g, b) = self;
        RGB(r / s, g / s, b / s)
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

/// scalar multiplication
impl<S: Mul<Output = S> + Copy> Mul<S> for RGB<S> {
    type Output = Self;
    fn mul(self, s: S) -> Self {
        let RGB(r, g, b) = self;
        RGB(r * s, g * s, b * s)
    }
}
