//! The scheduler handles a set of launchers, deciding who to launch and who
//! to wait for.

use crate::error::Error;
use crate::yaml::Yaml;

use crate::args::Args;
use crate::input::{Input, Inputter};
use crate::launcher::Launcher;
use crate::output::Output;

use std::fs;
use std::path::Path;
use std::time::Duration;

use rayon::prelude::*;

pub struct Scheduler {
    launchers: Vec<Launcher>,
}

impl Scheduler {
    fn dispatch_file(file: &Path) -> Result<Input, Error> {
        match file.extension() {
            Some(os_str) => match os_str.to_str() {
                Some("yml") | Some("yaml") => Yaml::parse(&fs::read_to_string(file)?),
                Some(ext) => Err(Error::UnhandledFileFormat(String::from(ext))),
                None => Err(Error::NoExtension),
            },
            None => Err(Error::NoExtension),
        }
    }

    pub fn from_args(args: &Args) -> Result<Scheduler, Error> {
        let mut launchers = Vec::new();

        for file in &args.files {
            let input = Scheduler::dispatch_file(file)?;

            for test_case in input.tests {
                launchers.push(Launcher::new(
                    test_case.name,
                    test_case.binary,
                    // If there are no arguments, pass an empty vector
                    test_case.args.unwrap_or_default(),
                    test_case.stdout,
                    test_case.stderr,
                    test_case.exit_code.map(|v| v as i32),
                    test_case.timeout.map(Duration::from_secs),
                ));
            }
        }

        Ok(Scheduler { launchers })
    }

    pub fn run(self) -> Result<Vec<Output>, Error> {
        self.launchers
            .into_par_iter()
            .map(|launcher| launcher.run())
            .collect()
    }
}
