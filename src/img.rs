use super::RGB;
use std::fs::File;
use std::io;
use std::path::Path;
/// A rectangular image on N pixels.
#[derive(Clone, Debug, PartialEq)]
pub struct Img<P> {
    pub buf: Vec<P>,
    pub width: u32,
}

impl<P> Img<P> {
    fn idx(&self, (x, y): (u32, u32)) -> usize {
        ((y * self.width) + x) as usize
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, (x, y): (u32, u32)) -> Option<&P> {
        self.buf.get(self.idx((x, y)))
    }
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.len() as u32 / self.width as u32)
    }
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
}

impl Img<RGB<u8>> {
    /// the raw_buf flattens out each RGB triplet;
    /// ```
    /// use dither::img::*;
    /// let img: Img<RGB<u8>> = Img{buf: vec![RGB(0, 1, 2), RGB(1, 1, 1)], width: 1};
    /// assert_eq!(img.raw_buf(), vec![0, 1, 2, 1, 1, 1]);
    /// ```
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
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        let w = io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, width, height);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.raw_buf()).unwrap();
        Ok(())
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
