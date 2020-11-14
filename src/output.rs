//! This module specifies the output format of an `ft` test case
//! To add an output format, simply implement the FtOutput trait. This is what
//! is done by YmlOutput and JsonOutput for example

use std::time::Duration;

use serde::Serialize;

pub static INVALID_EXIT: i32 = std::i32::MIN;

#[derive(Debug, Serialize)]
pub struct Output {
    exit_code: i32,
    stdout: String,
    stderr: String,
    time: Duration,
}

impl Output {
    pub fn new(exit_code: i32, stdout: String, stderr: String, time: Duration) -> Output {
        Output {
            exit_code,
            stdout,
            stderr,
            time,
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    pub fn out(&self) -> &String {
        &self.stdout
    }

    pub fn err(&self) -> &String {
        &self.stderr
    }

    pub fn time(&self) -> &Duration {
        &self.time
    }
}

// FIXME: Should take an Output and format it to whatever format wanted
pub trait FtOutput {
    /// Format the Output results to a String
    fn fmt(data: &Output) -> String;
}
