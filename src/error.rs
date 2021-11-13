use std::ffi::OsString;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unhandled file format: {0:?}")]
    UnhandledFormat(OsString),
    #[error("invalid file name: {0:?}")]
    InvalidFileName(OsString),
}
