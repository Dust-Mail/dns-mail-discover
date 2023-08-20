use std::{error, fmt::Display, io::Error as IoError, result};

use trust_dns_resolver::error::ResolveError;

macro_rules! impl_from_error {
    ($error_type:ty, $error_kind:expr, $error_msg:expr) => {
        impl From<$error_type> for Error {
            fn from(err: $error_type) -> Self {
                Error::new($error_kind(err), $error_msg)
            }
        }
    };
}

macro_rules! err {
    ($kind:expr, $($arg:tt)*) => {{
		use crate::error::Error;

        let kind = $kind;
        let message = format!($($arg)*);
        return Err(Error::new( kind, message ));
    }};
}

#[derive(Debug)]
pub enum ErrorKind {
    NoBytesSent,
    Unresolvable,
    NotFound,
    Io(IoError),
    Resolve(ResolveError),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl_from_error!(
    ResolveError,
    |err| ErrorKind::Resolve(err),
    "Failed to resolve dns query"
);
impl_from_error!(IoError, |err| ErrorKind::Io(err), "IO error");

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

pub(crate) use err;

pub type Result<T> = result::Result<T, Error>;
