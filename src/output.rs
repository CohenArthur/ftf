//! This module specifies the output format of an `ft` test case
//! To add an output format, simply implement the FtOutput trait. This is what
//! is done by YmlOutput and JsonOutput for example

// FIXME: Add more documentation

use std::time::Duration;

use crate::args::FtArgs;
use crate::exp_got::ExpGot;

use colored::Colorize;
use serde::Serialize;

pub static INVALID_EXIT: i32 = 1;

#[derive(Debug, Serialize)]
pub struct Output {
    name: String,
    exit_code: ExpGot<i32>,
    stdout: ExpGot<String>,
    stderr: ExpGot<String>,
    time: ExpGot<Duration>,
}

impl Output {
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

    // FIXME: Doc
    pub fn valid(&self) -> bool {
        let mut retval = self.exit_code.expected.unwrap_or(0) == self.exit_code.got;

        retval = if retval {
            match &self.stdout.expected {
                Some(s) => s == &self.stdout.got,
                None => true,
            }
        } else {
            retval
        };

        retval = if retval {
            match &self.stderr.expected {
                Some(s) => s == &self.stderr.got,
                None => true,
            }
        } else {
            retval
        };

        retval = if retval {
            match self.time.expected {
                Some(s) => s == self.time.got,
                None => true,
            }
        } else {
            retval
        };

        retval
    }

    // FIXME: Use args
    pub fn display(&self, args: &FtArgs, retval: &mut i32) {
        let mut res_string = "OK".green();
        let is_valid = self.valid();

        if !is_valid {
            *retval = INVALID_EXIT;
            res_string = "KO".red();
        }

        println!("{}: [{}]", self.name, res_string);
        if !is_valid {
            eprintln!("{}", args.get_formatter()(self));
        }
    }

    #[cfg(test)]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[cfg(test)]
    pub fn exit_code(&self) -> i32 {
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
