use failure::Fail;
use std::convert::From;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "i/o error: {}", _0)]
    Io(std::io::Error),

    #[fail(display = "file cache error: {}", _0)]
    FileCache(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}
