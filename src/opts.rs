use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt, PartialEq, Clone)]
#[structopt(name = "dither")]
pub struct Opt {
    /// Provide verbose debug information. Default is false.
    #[structopt(short = "v", long = "verbose")]
    pub debug: bool,
    /// Input file: currently, only .png is supported.
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    /// Whether to use color. Default is false.
    #[structopt(short = "c", long = "color")]
    pub color: bool,

    /// Color depth. Must be between 1 and 8.
    #[structopt(short = "b", long = "bit_depth", default_value = "1")]
    pub bit_depth: u8,

    /// Output file: will be written to as a .png
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,

    /// Ditherering algorithm to use. Options are
    /// "floyd", "atkinson", "stucki", "burkes","jarvis"
    #[structopt(short = "d", long = "dither", default_value = "floyd")]
    pub ditherer: super::Ditherer,
}
