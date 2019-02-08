use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt, PartialEq, Clone)]
#[structopt(name = "dither")]
pub struct Opt {
    /// Provide verbose debug information. Default is false.
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
    /// Input file. Supported file types:
    /// `PNG`
    /// `JPEG`
    /// `GIF`
    /// `BMP`
    /// `ICO`
    /// `TIFF`
    #[structopt(name = "input", parse(from_os_str))]
    pub input: PathBuf,
    /// Whether to use color. Default is false.
    ///  Mutually exclusive with the palette ("-p, --palette") option.
    #[structopt(short = "c", long = "color")]
    pub color: bool,

    /// Color depth. Must be between 1 and 8.
    #[structopt(short = "b", long = "bit_depth", default_value = "1")]
    pub bit_depth: u8,

    /// Output file: will be written to as a .png or .jpg (inferred from file extension) If left empty,
    /// it will be saved to a .png with the follwing information:
    /// $BASE_dithered_$DITHER_$COLOR_$DEPTH
    /// `$dither bunny.png -c --dither=atkinson --bit-depth=2` will save to bunny_atkinson_c_2.png
    #[structopt(name = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Ditherering algorithm to use. Options are
    /// "floyd", "atkinson", "stucki", "burkes","jarvis"
    #[structopt(short = "d", long = "dither", default_value = "floyd")]
    pub ditherer: super::Ditherer,

    // Color palette to use in black and white mode; input must be a pair of hexademical numbers,
    // the first being the foreground, the second the background.
    // i.e, "0xFF0000, 0x000000))". Mutually exclusive with the color ("-c, --color") option.
    #[structopt(short = "p", long = "palette")]
    pub palette: Option<super::Palette>,
}

impl Opt {
    pub fn output_path(&self) -> std::borrow::Cow<PathBuf> {
        if let Some(output) = &self.output {
            std::borrow::Cow::Borrowed(output)
        } else {
            let output_stem = self
                .input
                .file_stem()
                .unwrap_or_else(|| self.input.as_ref()) // no extension; use the whole path
                .to_string_lossy();
            std::borrow::Cow::Owned(std::path::PathBuf::from(format!(
                "{base}_dithered_{dither}_{color}_{depth}.png",
                base = output_stem,
                dither = self.ditherer,
                color = if self.color { "c" } else { "bw" },
                depth = self.bit_depth,
            )))
        }
    }
}
