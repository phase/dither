use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt, PartialEq, Clone)]
#[structopt(name = "basic")]
pub struct Opt {
    /// provide verbose debug information: default is false
    #[structopt(short = "v", long = "debug")]
    pub debug: bool,
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    /// Whether to use color. Default is false.
    #[structopt(short = "c", long = "color")]
    pub color: bool,
    /// Output file
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,

    /// Ditherer: default is floyd_steinberg
    #[structopt(short = "d", long = "dither", default_value = "floyd")]
    pub ditherer: super::Ditherer,
}
