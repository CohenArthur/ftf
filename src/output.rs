//! This module specifies the output format of an `ft` test case
//! To add an output format, simply implement the FtOutput trait. This is what
//! is done by YmlOutput and JsonOutput for example

use std::time::Duration;

use crate::error::Error;
use crate::expected::{ExpGot, ExpString};
use crate::log;

use serde::Serialize;

#[derive(Debug, Serialize)]
/// Output structure, contains what was expected and what has been output by the
/// command
pub struct Output {
    name: String,
    exit_code: ExpGot<i32>,
    stdout: ExpString,
    stderr: ExpString,
    time: ExpGot<Duration>,
}

impl Output {
    /// Create a new output using `ExpGot`s.
    pub fn new(
        name: String,
        exit_code: ExpGot<i32>,
        stdout: ExpString,
        stderr: ExpString,
        time: ExpGot<Duration>,
    ) -> Output {
        let out = Output {
            name,
            exit_code,
            stdout,
            stderr,
            time,
        };

        if out.exit_code.eq() {
            log::success(&out.name);
        } else {
            log::failure(&out.name);
        }

        out
    }

    pub fn is_valid(&self) -> bool {
        self.exit_code().eq() && self.stdout.matches() && self.stderr.matches() && self.time.eq()
    }

    /// Display the output of a command accordingly, with the following format:
    ///
    /// <test name>: [ OK | KO ]
    ///
    /// In case of KO, the complete output will be dumped using the format passed
    /// to `ft` with the `-o|--output` argument

    pub fn exit_code(&self) -> &ExpGot<i32> {
        &self.exit_code
    }

    #[cfg(test)]
    pub fn exit_code_got(&self) -> i32 {
        self.exit_code.got
    }

    #[cfg(test)]
    pub fn out(&self) -> &str {
        &self.stdout.got
    }

    #[cfg(test)]
    pub fn err(&self) -> &str {
        &self.stderr.got
    }

    #[cfg(test)]
    pub fn time(&self) -> &Duration {
        &self.time.got
    }
}

pub trait FtOutput {
    /// Format the Output results to a String
    fn fmt(data: &Output) -> Result<String, Error>;
}
