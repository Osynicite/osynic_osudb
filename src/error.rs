use nom::{
    Err as NomErr, Needed,
    error::{Error as NomError, ErrorKind as NomErrorKind},
};

pub type Result<T> = core::result::Result<T, Error>;

// Error

pub struct Error {
    inner: Box<ErrorKind>,
}

impl Error {
    pub fn new(inner: ErrorKind) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}

// ErrorKind

pub enum ErrorKind {
    LzmaError(liblzma::stream::Error),
    IoError(std::io::Error),
    ParseError(NomErrorKind),
    ParseIncomplete(Needed),
}

impl From<liblzma::stream::Error> for Error {
    fn from(e: liblzma::stream::Error) -> Self {
        Error::new(ErrorKind::LzmaError(e))
    }
}

impl From<NomErr<NomError<&[u8]>>> for Error {
    fn from(err: NomErr<NomError<&[u8]>>) -> Self {
        match err {
            NomErr::Error(e) => Error::new(ErrorKind::ParseError(e.code)),
            NomErr::Failure(e) => Error::new(ErrorKind::ParseError(e.code)),
            NomErr::Incomplete(needed) => Error::new(ErrorKind::ParseIncomplete(needed)),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::new(ErrorKind::IoError(e))
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::LzmaError(e) => write!(f, "LZMA compression error: {e}"),
            ErrorKind::IoError(e) => write!(f, "IO error: {e}"),
            ErrorKind::ParseError(e) => write!(f, "Parse error: {e:?}"),
            ErrorKind::ParseIncomplete(needed) => write!(f, "Parse incomplete: {needed:?}"),
        }
    }
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::LzmaError(e) => write!(f, "LZMA compression error: {e}"),
            ErrorKind::IoError(e) => write!(f, "IO error: {e}"),
            ErrorKind::ParseError(e) => write!(f, "Parse error: {e:?}"),
            ErrorKind::ParseIncomplete(needed) => write!(f, "Parse incomplete: {needed:?}"),
        }
    }
}
