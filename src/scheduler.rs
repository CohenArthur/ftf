//! The scheduler handles a set of launchers, deciding who to launch and who
//! to wait for.

use crate::yaml::Yaml;

use crate::input::{Input, Inputter};
use crate::launcher::Launcher;
use crate::output::Output;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub struct Scheduler {
    launchers: Vec<Launcher>,
}

impl Scheduler {
    fn dispatch_file(file: &Path) -> Input {
        // FIXME: Cleanup
        match file.extension().unwrap().to_str() {
            Some("yml") | Some("yaml") => Yaml::parse(&fs::read_to_string(file).unwrap()),
            Some(_) => panic!("Unhandled file format {}", file.to_str().unwrap()),
            None => panic!("Not a valid filename {}", file.to_str().unwrap()),
        }
    }

    pub fn from_files(files: &[PathBuf]) -> Scheduler {
        let mut launchers = Vec::new();

        for file in files {
            let input = Scheduler::dispatch_file(&file);

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

        Scheduler { launchers }
    }

    pub fn run(&self) -> Vec<Output> {
        // FIXME: Don't unwrap
        // FIXME: actually implement a cool scheduler (parallelism, CFS (red-black trees), etc...)
        self.launchers
            .iter()
            .map(|launcher| launcher.run().unwrap())
            .collect()
    }
}
