use std::io::Error as IoError;
use std::result::Result as StdResult;

/// Common result type throughout the library.
pub type Result<T> = StdResult<T, IoError>;
