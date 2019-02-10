//! # Dither
//!s
//! Inspired by: <http://www.tannerhelland.com/4660/dithering-eleven-algorithms-source-code/>

pub mod dither;

pub mod color;
mod error;
mod img;
mod opts;
#[cfg(test)]
mod tests;
#[macro_use]
extern crate lazy_static;

pub fn clamp_f64_to_u8(n: f64) -> u8 {
    match n {
        n if n > 255.0 => 255,
        n if n < 0.0 => 0,
        n => n as u8,
    }
}
pub use self::error::{Error, Result};
use self::{
    color::{Palette, CGA, RGB},
    dither::Ditherer,
    img::Img,
    opts::Opt,
};

use structopt::StructOpt;

fn main() -> Result<()> {
    let opts = Opt::from_args();
    _main(&opts)
}

fn _main(opts: &Opt) -> Result<()> {
    let debug = |msg: &str| {
        if opts.verbose {
            eprintln!("{}", msg);
        }
    };

    debug("program start");
    let img: Img<RGB<f64>> =
        Img::<RGB<u8>>::load(&opts.input)?.convert_with(|rgb| rgb.convert_with(f64::from));
    let quantize = create_quantize_n_bits_func(opts.bit_depth)?;

    let output_img = match opts.color_mode {
        color::Mode::CGA | color::Mode::SingleColor(_) | color::Mode::CustomPalette { .. }
            if opts.bit_depth > 1 =>
        {
            return Err(Error::IncompatibleOptions);
        }

        color::Mode::Color => {
            debug("color mode");
            opts.ditherer
                .dither(img, RGB::map_across(quantize))
                .convert_with(|rgb| rgb.convert_with(clamp_f64_to_u8))
        }

        color::Mode::CGA => {
            debug("cga mode");
            opts.ditherer
                .dither(img, CGA::quantize)
                .convert_with(|rgb| rgb.convert_with(clamp_f64_to_u8))
        }

        color::Mode::BlackAndWhite => {
            debug("black and white mode!");
            let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());
            opts.ditherer
                .dither(bw_img, quantize)
                .convert_with(RGB::from_chroma_corrected_black_and_white)
        }

        color::Mode::SingleColor(color) => {
            if opts.verbose {
                eprintln!("1bit color mode: {}", color)
            }
            let Palette { front, back } = color.into();
            let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());
            opts.ditherer
                .dither(bw_img, quantize)
                .convert_with(RGB::from_chroma_corrected_black_and_white)
                .convert_with(|rgb| if rgb == RGB(0, 0, 0) { front } else { back })
        }

        color::Mode::CustomPalette { front, back } => {
            debug("paletted 1bit mode");
            let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());
            opts.ditherer
                .dither(bw_img, quantize)
                .convert_with(RGB::from_chroma_corrected_black_and_white)
                .convert_with(|rgb| if rgb == RGB(0, 0, 0) { front } else { back })
        }
    };

    output_img.save(opts.output_path().as_ref())?;
    debug(&format!(
        "saved to {}",
        opts.output_path().to_string_lossy()
    ));
    Ok(())
}

/// quantize to n bits
/// ```
/// # use dither::create_quantize_n_bits_func;
/// let quantize_1_bit = |n: u8| if n > 127 {255, 255-n} else {0, n};
/// let quantization_func = create_quantize_n_bits_func(1);
/// assert_eq!(quantize_1_bit(5), create_quantize_n_bits_func(1)(5));
/// ```
pub fn create_quantize_n_bits_func(n: u8) -> Result<impl FnMut(f64) -> (f64, f64)> {
    if n == 0 || n > 7 {
        Err(Error::BadBitDepth(n))
    } else {
        Ok(move |x: f64| {
            let step_size = 256. / f64::from(n);

            let floor = f64::floor(x / step_size) * step_size;
            let floor_rem = x - floor;

            let ceil = f64::ceil(x / step_size) * step_size;
            let ceil_rem = ceil - x;

            if floor_rem < ceil_rem {
                let quot = f64::max(floor, 0.0);
                (quot, floor_rem)
            } else {
                let quot = f64::min(255.0, ceil);
                (quot, -ceil_rem)
            }
        })
    }
}
