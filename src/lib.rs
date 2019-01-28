mod rgb;
pub use self::rgb::{RGB, RGBA};
use std::ops::{Add, Div, Mul};

#[derive(Clone, Debug, PartialEq)]
/// A rectangular image on N pixels.
pub struct Img<P> {
    pub buf: Vec<P>,
    pub width: u32,
}

pub trait Dither {
    const DIV: isize;
    const OFFSETS: &'static [(isize, isize, isize)];
    /// dither a 2d matrx
    fn dither<P>(buf: &Img<P>, quantize: impl Fn(P) -> (P, P)) -> Img<P>
    where
        P: Add<Output = P> + Mul<Output = P> + Div<Output = P> + Default + Copy + From<isize>,
    {
        {
            let (width, height) = buf.size();

            let mut output = vec![P::default(); buf.len()];
            let mut spillover = vec![P::default(); buf.len()];

            for x in 0..width {
                for y in 0..height {
                    let i = (y * width + x) as usize;

                    let p = buf[(y, x)] + spillover[i];
                    let (q, r) = quantize(p);

                    output[i] = q;

                    // add spillover matrices
                    let (y, x) = (y as isize, x as isize);
                    for (dy, dx, mul) in Self::OFFSETS.iter().cloned() {
                        let i = ((y + dy) * width as isize + (x + dx)) as usize;

                        if let Some(v) = spillover.get_mut(i) {
                            *v = *v + ((r * mul.into()) / Self::DIV.into());
                        }
                    }
                }
            }
            Img {
                buf: output,
                width: buf.width,
            }
        }
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

impl Dither for Atkinson {
    const DIV: isize = 8;
    const OFFSETS: &'static [(isize, isize, isize)] = &[
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
impl Dither for Stucki {
    const DIV: isize = 42;
    const OFFSETS: &'static [(isize, isize, isize)] = &[
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

pub fn dither<P, D>(buf: &Img<P>, quantize: impl Fn(P) -> (P, P)) -> Img<P>
where
    P: Add<Output = P> + Mul<Output = P> + Div<Output = P> + Default + Copy + From<isize>,
    D: Dither,
{
    let (width, height) = buf.size();

    let mut output = vec![P::default(); buf.len()];
    let mut spillover = vec![P::default(); buf.len()];

    for x in 0..width {
        for y in 0..height {
            let i = (y * width + x) as usize;

            let p = buf[(y, x)] + spillover[i];
            let (q, r) = quantize(p);

            output[i] = q;

            // add spillover matrices
            let (y, x) = (y as isize, x as isize);
            for (dy, dx, mul) in D::OFFSETS.iter().cloned() {
                let i = ((y + dy) * width as isize + (x + dx)) as usize;

                if let Some(v) = spillover.get_mut(i) {
                    *v = *v + ((r * mul.into()) / D::DIV.into());
                }
            }
        }
    }
    Img {
        buf: output,
        width: buf.width,
    }
}
fn black_white_quantize(p: f64) -> (f64, f64) {
    debug_assert!(0. < p && p < 1.);
    if p < 0.5 {
        (0., p)
    } else {
        (1., -(p - 0.5))
    }
}

#[inline]
fn idx((x, y): (u32, u32)) -> usize {
    (y * x + x) as usize
}
impl<P> Img<P> {
    pub fn height(&self) -> u32 {
        self.buf.len() as u32 / self.width
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn get(&self, i: (u32, u32)) -> Option<&P> {
        self.buf.get(idx(i))
    }
    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    pub fn new(width: u32, height: u32) -> Self
    where
        P: Default + Clone,
    {
        Img {
            buf: vec![P::default(); (width * height) as usize],
            width,
        }
    }

    pub fn from_vec_and_width(buf: Vec<P>, width: u32) -> Option<Self> {
        if buf.len() % width as usize != 0 {
            None
        } else {
            Some({ Img { buf, width } })
        }
    }

    pub unsafe fn from_raw_vec_and_width(buf: Vec<P>, width: u32) -> Self {
        Img { buf, width }
    }

    fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub fn iter_indices(&self) -> Griderator<P>
    where
        P: Copy,
    {
        Griderator { buf: self, i: 0 }
    }

    pub fn iter(&self) -> std::slice::Iter<P> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<P> {
        self.into_iter()
    }
}

impl<P> IntoIterator for Img<P> {
    type Item = P;
    type IntoIter = std::vec::IntoIter<P>;
    fn into_iter(self) -> Self::IntoIter {
        self.buf.into_iter()
    }
}

impl<'a, P> IntoIterator for &'a Img<P> {
    type Item = &'a P;
    type IntoIter = std::slice::Iter<'a, P>;
    fn into_iter(self) -> Self::IntoIter {
        self.buf.iter()
    }
}

impl<'a, P> IntoIterator for &'a mut Img<P> {
    type Item = &'a mut P;
    type IntoIter = std::slice::IterMut<'a, P>;
    fn into_iter(self) -> Self::IntoIter {
        self.buf.iter_mut()
    }
}

impl<P> std::ops::Index<(u32, u32)> for Img<P> {
    type Output = P;
    fn index(&self, (width, height): (u32, u32)) -> &P {
        let i = height * width + width;
        &self.buf[i as usize]
    }
}

impl<P> std::ops::IndexMut<(u32, u32)> for Img<P> {
    fn index_mut(&mut self, (width, height): (u32, u32)) -> &mut P {
        let i = height * width + width;
        &mut self.buf[i as usize]
    }
}

pub struct Griderator<'a, P: Copy> {
    buf: &'a Img<P>,
    i: u32,
}

impl<'a, P: Copy> Iterator for Griderator<'a, P> {
    type Item = (u32, u32, P);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i as usize >= self.buf.len() {
            None
        } else {
            let y = self.i / self.buf.width;
            let x = self.i % self.buf.width;
            Some((x, y, self.buf.buf[self.i as usize]))
        }
    }
}
