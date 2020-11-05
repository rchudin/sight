mod data;

pub use self::data::IncorrectData;

#[derive(Debug)]
pub enum Error {
    IncorrectDataError(IncorrectData),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::IncorrectDataError(ref e) => e.fmt(f),
        }
    }
}
