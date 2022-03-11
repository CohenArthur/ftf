//! Transforms the written test cases from whatever data language was used
//! to conditions that `ft` understands. Add any language by implementing
//! the Inputter trait

use crate::error::Error;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct TestCase {
    pub name: String,
    pub binary: String,
    pub args: Option<Vec<String>>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub exit_code: Option<u8>,
    pub timeout: Option<u64>,
}

#[derive(Deserialize)]
pub struct Input {
    pub tests: Vec<TestCase>,
}

pub trait Inputter {
    fn parse(data: &str) -> Result<Input, Error>;
}
