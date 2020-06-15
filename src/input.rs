//! Transforms the written test cases from whatever data language was used
//! to conditions that `ft` understands. Add any language by implementing
//! the FtInput trait

use std::time::Duration;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Input {
    pub name: String,
    pub cmd: String,
    pub args: Option<Vec<String>>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub exit_code: Option<u8>,
    pub timeout: Option<Duration>,
}

pub trait FtInput {
    fn parse(data: &str) -> Input;
}
