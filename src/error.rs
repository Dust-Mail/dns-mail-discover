use std::{error, fmt::Display, io::Error as IoError, result};

use trust_dns_resolver::error::ResolveError;

#[derive(Debug)]
pub enum ErrorKind {
    NoBytesSent,
    Unresolvable,
    Io(IoError),
    Resolve(ResolveError),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl From<ResolveError> for Error {
    fn from(error: ResolveError) -> Self {
        Error::new(ErrorKind::Resolve(error), "Dns error")
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::new(ErrorKind::Io(error), "IO error")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error {
    pub fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl error::Error for Error {}

#[macro_export]
macro_rules! failed {
    ($kind:expr, $($arg:tt)*) => {{
		use crate::error::Error;

        let kind = $kind;
        let message = format!($($arg)*);
        return Err(Error::new( kind, message ));
    }};
}

pub type Result<T> = result::Result<T, Error>;
