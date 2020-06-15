//! This module specifies the output format of an `ft` test case
//! To add an output format, simply implement the FtOutput trait. This is what
//! is done by YmlOutput and JsonOutput for example

use std::time::Duration;

use serde::Serialize;

#[derive(Serialize)]
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

// FIXME: Should take an Output and format it to whatever format wanted
pub trait FtOutput {
    /// Format the Output results to a String
    fn fmt(data: &Output) -> String;
}
