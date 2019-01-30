use std::ops::{Add, Div, Mul};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RGB<N>(pub N, pub N, pub N);

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
