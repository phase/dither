#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Image(image::ImageError),
    BadBitDepth(u8),
    Palette(super::rgb::PaletteError),
    IncompatibleOptions,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Image(err) => write!(f, "image library error: {}", err),
            Error::IO(err) => write!(f, "io error: {}", err),
            Error::BadBitDepth(n) => write!(f, "bit depth must be between 1 and 7, but was {}", n),
            Error::Palette(err) => err.fmt(f),
            Error::IncompatibleOptions => write!(
                f,
                "the palette (-p, --palette) option is mututally exclusive with the bit depth and color options."
            ),
        }
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::Image(err)
    }
}
