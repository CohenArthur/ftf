//! This module specifies the output format of an `ft` test case
//! To add an output format, simply implement the FtOutput trait. This is what
//! is done by YmlOutput and JsonOutput for example

use std::time::Duration;

use crate::{exp_got::ExpGot, Formatter};

use colored::Colorize;
use serde::Serialize;

pub static INVALID_EXIT: i32 = 1;

#[derive(Debug, Serialize)]
/// Output structure, contains what was expected and what has been output by the
/// command
pub struct Output {
    name: String,
    exit_code: ExpGot<i32>,
    stdout: ExpGot<String>,
    stderr: ExpGot<String>,
    time: ExpGot<Duration>,
}

impl Output {
    /// Create a new output using `ExpGot`s.
    pub fn new(
        name: String,
        exit_code: ExpGot<i32>,
        stdout: ExpGot<String>,
        stderr: ExpGot<String>,
        time: ExpGot<Duration>,
    ) -> Output {
        Output {
            name,
            exit_code,
            stdout,
            stderr,
            time,
        }
    }

    fn valid(&self) -> bool {
        let mut retval = self.exit_code.expected.unwrap_or(0) == self.exit_code.got;

        retval = if retval { self.stdout.eq() } else { retval };
        retval = if retval { self.stderr.eq() } else { retval };
        retval = if retval { self.time.eq() } else { retval };

        retval
    }

    /// Display the output of a command accordingly, with the following format:
    ///
    /// <test name>: [ OK | KO ]
    ///
    /// In case of KO, the complete output will be dumped using the format passed
    /// to `ft` with the `-o|--output` argument
    pub fn display(&self, formatter: &Formatter, retval: &mut i32) {
        let mut res_string = "OK".green();
        let is_valid = self.valid();

        if !is_valid {
            *retval = INVALID_EXIT;
            res_string = "KO".red();
        }

        println!("[{}] {}", res_string, self.name);
        if !is_valid {
            eprintln!("{}", formatter(self));
        }
    }

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
    fn fmt(data: &Output) -> String;
}
