//! # Dither
//!s
//! Inspired by: <http://www.tannerhelland.com/4660/dithering-eleven-algorithms-source-code/>

pub mod dither;

mod error;
mod img;
mod opts;
mod rgb;
#[cfg(test)]
mod tests;
pub use self::error::{Error, Result};
use self::{dither::Ditherer, img::Img, opts::Opt, rgb::RGB};

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
    let img: Img<RGB<f64>> = Img::read_png(&opts.input)?;
    let quantize = create_quantize_n_bits_func(opts.bit_depth)?;

    let output_img = if opts.color {
        debug("color printing");
        opts.ditherer
            .dither(img, RGB::map_across(quantize))
            .convert_with(|rgb| rgb.convert_with(rgb::clamp_f64_to_u8))
    } else {
        debug("black and white printing");
        let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());
        opts.ditherer
            .dither(bw_img, quantize)
            .convert_with(RGB::from_chroma_corrected_black_and_white)
    };

    let output = opts.output_path();
    output_img.save_png(output.as_ref())?;
    debug(&format!("saved to {}", output.to_string_lossy()));
    Ok(())
}

/// quantize to n bits
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
                (f64::max(floor, 0.0), floor_rem)
            } else {
                (f64::min(255.0, ceil), -ceil_rem)
            }
        })
    }
}
