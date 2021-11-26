use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid formatter used: {0}")]
    InvalidFormatter(String),
}
