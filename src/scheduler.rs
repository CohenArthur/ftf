//! The scheduler handles a set of launchers, deciding who to launch and who
//! to wait for.

use crate::yaml::Yaml;
use crate::args::FtArgs;
use crate::error::Error;
use crate::input::{FtInput, Input};
use crate::launcher::Launcher;
use crate::output::Output;

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub struct Scheduler {
    launchers: Vec<Launcher>,
}

impl Scheduler {
    fn dispatch_file(file: &PathBuf) -> Result<Input, Error> {
        // FIXME: Cleanup
        match file.extension().unwrap().to_str() {
            Some("yml") | Some("yaml") => Ok(Yaml::parse(&fs::read_to_string(file).unwrap())),
            Some(_) => Err(Error::UnhandledFormat(file.clone().into_os_string())),
            None => Err(Error::InvalidFileName(file.clone().into_os_string())),
        }
    }

    pub fn from_args(args: &FtArgs) -> Result<Scheduler, Error> {
        let mut launchers = Vec::new();

        for file in &args.files {
            let input = Scheduler::dispatch_file(&file)?;

            for test_case in input.tests {
                launchers.push(Launcher::new(
                    test_case.name,
                    test_case.binary,
                    test_case.args,
                    test_case.stdout,
                    test_case.stderr,
                    test_case.exit_code.map(|v| v as i32),
                    test_case.timeout.map(Duration::from_secs),
                ));
            }
        }

        Ok(Scheduler { launchers })
    }

    pub fn run(&self) -> Vec<Output> {
        // FIXME: Don't unwrap
        self.launchers
            .iter()
            .map(|launcher| launcher.run().unwrap())
            .collect()
    }
}
