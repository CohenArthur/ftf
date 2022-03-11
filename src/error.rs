use std::convert::From;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(std::io::Error),
    #[error("unhandled file format: {0}")]
    UnhandledFileFormat(String),
    #[error("invalid file name: no extension")]
    NoExtension,
    #[error("couldn't (de)serialize YAML data: {0}")]
    SerdeYaml(serde_yaml::Error),
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Error {
        Error::SerdeYaml(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}
