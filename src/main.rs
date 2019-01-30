//!
//!
//!
//!
pub mod dither;
pub use self::dither::{Dither, Ditherer};
mod img;
mod opts;
mod rgb;
#[cfg(test)]
mod tests;
#[macro_use]
extern crate structopt;
pub use self::img::Img;
pub use self::opts::Opt;
pub use self::rgb::RGB;

use structopt::StructOpt;

fn main() {
    let opts = Opt::from_args();
    let debug = opts.debug;
    let (input, output) = (opts.input.clone(), opts.output.clone());
    let ditherer_type = format!("{:?}", &opts.ditherer);
    match _main(opts) {
        Ok(()) if debug => eprintln!(
            "dithered {} using {:?}; saved to {}",
            input.to_string_lossy(),
            ditherer_type,
            output.to_string_lossy(),
        ),
        Ok(()) => {}
        Err(err) => eprintln!("{}", err),
    }
}
fn _main(opts: Opt) -> std::io::Result<()> {
    let Img { buf, width } = Img::read_png(opts.input)?;
    let output_buf = if opts.color {
        // transform to 16-bit RGB
        let rgb_img = Img {
            buf: buf.into_iter().map(RGB::<i16>::from).collect(),
            width,
        };
        // dither to 1bit in each channel
        let dithered = opts.ditherer.dither(rgb_img, four_color);
        // transform back to 8bit RGB
        dithered.buf.into_iter().map(RGB::<u8>::from).collect()
    } else {
        // transform to 16 bit black and white
        let bw_img = Img {
            buf: buf
                .into_iter()
                .map(RGB::to_chroma_corrected_black_and_white)
                .map(i16::from)
                .collect(),
            width,
        };
        // dither to 1-bit
        let dithered = opts.ditherer.dither(bw_img, quantize_1bit);
        // transform back to 8bit RBG
        dithered
            .buf
            .into_iter()
            .map(RGB::from_color_corrected_black_and_white)
            .collect()
    };
    let output_img = Img {
        buf: output_buf,
        width,
    };
    output_img.save_png(opts.output)
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
