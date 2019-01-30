use std::path::PathBuf;
#[derive(Debug, StructOpt, PartialEq, Clone)]

pub struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    /// Output file
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,

    /// Ditherer: default is floyd_steinberg
    #[structopt(default_value = "floyd")]
    pub ditherer: super::Ditherer,

    #[structopt(default_value = "4")]
    pub bit_depth: u8,
}
