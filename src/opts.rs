use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt, PartialEq, Clone)]
#[structopt(name = "dither")]
pub struct Opt {
    /// Provide verbose debug information. Default is false.
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
    /// Input file: currently, only .png is supported.
    #[structopt(name = "input", parse(from_os_str))]
    pub input: PathBuf,
    /// Whether to use color. Default is false.
    #[structopt(short = "c", long = "color")]
    pub color: bool,

    /// Color depth. Must be between 1 and 8.
    #[structopt(short = "b", long = "bit_depth", default_value = "1")]
    pub bit_depth: u8,

    /// Output file: will be written to as a .png. If left empty,
    /// the input file will be appended with the follwing information:
    /// $BASE_dithered_$DITHER_$COLOR_$DEPTH
    /// `$dither bunny.png -c --dither=atkinson --bit-depth=2` will save to bunny_atkinson_c_2.png
    #[structopt(name = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Ditherering algorithm to use. Options are
    /// "floyd", "atkinson", "stucki", "burkes","jarvis"
    #[structopt(short = "d", long = "dither", default_value = "floyd")]
    pub ditherer: super::Ditherer,
}

impl Opt {
    pub fn output_path(&self) -> std::borrow::Cow<PathBuf> {
        if let Some(output) = &self.output {
            std::borrow::Cow::Borrowed(output)
        } else {
            let output_stem = self
                .input
                .file_stem()
                .unwrap_or_else(|| self.input.as_ref())
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
