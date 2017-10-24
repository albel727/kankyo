use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

/// Common result type throughout the library.
pub type Result<T> = StdResult<T, Error>;

/// Standard and only Result enum for the crate. This is the `Result`'s `Err`
/// type for all public functions.
#[derive(Debug)]
pub enum Error {
    /// An error from the `std::io` module occurred.
    Io(IoError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref inner) => inner.description(),
        }
    }
}
