use std::ops::{Add, Div, Mul};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RGB<N>(N, N, N);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RGBA<N> {
    rgb: RGB<N>,
    a: N,
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

impl<N> Add for RGBA<N>
where
    N: Add<Output = N>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let RGBA { rgb: rgb0, a: a0 } = self;
        let RGBA { rgb: rgb1, a: a1 } = other;
        RGBA {
            rgb: rgb0 + rgb1,
            a: a0 + a1,
        }
    }
}

impl<N> Mul for RGBA<N>
where
    N: Mul<Output = N>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let RGBA { rgb: rgb0, a: a0 } = self;
        let RGBA { rgb: rgb1, a: a1 } = other;
        RGBA {
            rgb: rgb0 * rgb1,
            a: a0 * a1,
        }
    }
}

impl<N> Div for RGBA<N>
where
    N: Div<Output = N>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let RGBA { rgb: rgb0, a: a0 } = self;
        let RGBA { rgb: rgb1, a: a1 } = other;
        RGBA {
            rgb: rgb0 / rgb1,
            a: a0 / a1,
        }
    }
}
