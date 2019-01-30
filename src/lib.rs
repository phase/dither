pub mod ditherers;
mod opts;
mod rgb;
#[cfg(test)]
mod tests;
#[macro_use]
extern crate structopt;

pub use self::ditherers::{Atkinson, Ditherer, FloydSteinberg, Stucki};
pub use self::opts::Opt;
pub use self::rgb::RGB;
use std::fs::File;
use std::io;
use std::ops::{Add, Div, Mul};
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
/// A rectangular image on N pixels.
pub struct Img<P> {
    pub buf: Vec<P>,
    pub width: u32,
}

pub fn main(opts: Opt) {
    let Opt {
        debug,
        input,
        output,
        ditherer,
        bit_depth,
    } = opts;
    let input_img = Img::read_png(input).unwrap();

    let output_img = match ditherer {
        Ditherer::Atkinson => Atkinson::dither(input_img, four_color),
        Ditherer::FloydSteinberg => FloydSteinberg::dither(input_img, four_color),
        Ditherer::Stucki => Stucki::dither(input_img, four_color),
    };
    output_img.save_png(output);
}

impl Img<RGB<u8>> {
    pub fn read_png<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let decoder = png::Decoder::new(File::open(path)?);
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut raw_buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut raw_buf).unwrap();
        let mut bytes = raw_buf.into_iter();
        let mut buf = Vec::with_capacity(bytes.len() / 3);
        while let (Some(r), Some(g), Some(b)) = (bytes.next(), bytes.next(), bytes.next()) {
            buf.push(RGB(r, g, b))
        }
        let width = info.width;
        Ok(Img { buf, width })
    }

    fn raw_buf(self) -> Vec<u8> {
        let mut raw_buf = Vec::with_capacity(self.len() * 3);
        for RGB(r, g, b) in self.buf {
            raw_buf.push(r);
            raw_buf.push(g);
            raw_buf.push(b);
        }
        raw_buf
    }

    pub fn save_png<P: AsRef<Path>>(self, path: P) -> io::Result<()> {
        use png::HasParameters;
        let (width, height) = self.size();
        let file = File::create(path)?;
        let w = io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, width, height);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.raw_buf()).unwrap();
        Ok(())
    }
}
fn clamp_to_u8(n: i16) -> u8 {
    if n < 0 {
        0
    } else if n > 0xff {
        0xff
    } else {
        n as u8
    }
}

fn quantize_1bit(b: i16) -> (i16, i16) {
    if b < 0x7f {
        (0, b)
    } else {
        (0xff, b - 0xff)
    }
}
fn four_color(RGB(r, g, b): RGB<i16>) -> (RGB<i16>, RGB<i16>) {
    let (r_quot, r_rem) = quantize_1bit(r);
    let (g_quot, g_rem) = quantize_1bit(g);
    let (b_quot, b_rem) = quantize_1bit(b);
    (RGB(r_quot, g_quot, b_quot), RGB(r_rem, g_rem, b_rem))
}

/// A type of Dither. Available dithers are [Stucki], [Atkinson], [FloydSteinberg]
pub trait Dither {
    const DIV: i16;
    const OFFSETS: &'static [(isize, isize, i16)];
    /// dither a 2d matrix.
    /// `P` is the type of pixel ([`u8`], [RGB<f64, f64, f64>]);
    /// S is multiplible and divisble by a **S**CALAR
    /// but adds to ITSELF
    ///
    fn dither(
        img: Img<RGB<u8>>,
        mut quantize: impl FnMut(RGB<i16>) -> (RGB<i16>, RGB<i16>),
    ) -> Img<RGB<u8>> {
        let (width, height) = img.size();

        let img: Img<RGB<i16>> = Img {
            buf: img.buf.into_iter().map(RGB::from).collect(),
            width,
        };
        let mut output: Vec<RGB<i16>> = vec![RGB::<i16>::default(); img.len()];
        let mut spillover: Vec<RGB<i16>> = vec![RGB::<i16>::default(); img.len()];

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
        Img {
            buf: output
                .into_iter()
                .map(|RGB(r, g, b)| RGB(clamp_to_u8(r), clamp_to_u8(g), clamp_to_u8(b)))
                .collect(),
            width,
        }
    }
}
impl<P> Img<P> {
    fn idx(&self, (x, y): (u32, u32)) -> usize {
        ((y * self.width) + x) as usize
    }
    fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn get(&self, (x, y): (u32, u32)) -> Option<&P> {
        self.buf.get(self.idx((x, y)))
    }
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.len() as u32 / self.width as u32)
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
    fn index(&self, (x, y): (u32, u32)) -> &P {
        &self.buf[self.idx((x, y))]
    }
}

impl<P> std::ops::IndexMut<(u32, u32)> for Img<P> {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut P {
        let i = self.idx((x, y));
        &mut self.buf[i]
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
