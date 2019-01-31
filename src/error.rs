#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Encoding(png::EncodingError),
    Decoding(png::DecodingError),
    BadBitDepth(u8),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IO(err) => write!(f, "io error: {}", err),
            Error::Encoding(err) => write!(f, "png encoding error: {}", err),
            Error::Decoding(err) => write!(f, "png decoding error: {}", err),
            Error::BadBitDepth(n) => write!(f, "bit depth must be between 1 and 7, but was {}", n),
        }
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<png::EncodingError> for Error {
    fn from(err: png::EncodingError) -> Self {
        Error::Encoding(err)
    }
}

impl From<png::DecodingError> for Error {
    fn from(err: png::DecodingError) -> Self {
        Error::Decoding(err)
    }
}
