//! Transforms the written test cases from whatever data language was used
//! to conditions that `ft` understands. Add any language by implementing
//! the FtInput trait

use std::time::Duration;

pub struct Input {
    pub name: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub exit_code: u8,
    pub timeout: Duration,
}

pub trait FtInput {
    fn parse() -> Input;
}
