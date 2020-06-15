//! This module specifies the output format of an `ft` test case
//! To add an output format, simply implement the FtOutput trait. This is what
//! is done by YmlOutput and JsonOutput for example

use std::time::Duration;

pub struct Output {
    exit_code: u8, // FIXME: Really a u8 ?
    stdout: String,
    stderr: String,
    time: Duration,
}

impl Output {
    pub fn new(exit_code: u8, stdout: String, stderr: String, time: Duration) -> Output {
        Output {
            exit_code,
            stdout,
            stderr,
            time,
        }
    }
}

pub trait FtOutput {
    /// Format the Output results to a String
    fn fmt(&self) -> String;
}
