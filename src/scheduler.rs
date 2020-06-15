//! The scheduler handles a set of launchers, deciding who to launch and who
//! to wait for.

use crate::yaml::Yaml;

use crate::args::FtArgs;
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
    fn dispatch_file(file: &PathBuf) -> Input {
        // FIXME: Cleanup
        match file.extension().unwrap().to_str() {
            Some("yml") | Some("yaml") => Yaml::parse(&fs::read_to_string(file).unwrap()),
            Some(_) => panic!("Unhandled file format {}", file.to_str().unwrap()),
            None => panic!("Not a valid filename {}", file.to_str().unwrap()),
        }
    }

    pub fn from_args(args: &FtArgs) -> Scheduler {
        let mut launchers = Vec::new();

        for file in &args.files {
            let input = Scheduler::dispatch_file(&file);

            for test_case in input.tests {
                launchers.push(Launcher::new(
                    test_case.cmd,
                    test_case.args,
                    Duration::new(
                        match test_case.timeout {
                            Some(time) => time,
                            None => 0,
                        },
                        0,
                    ),
                ));
            }
        }

        Scheduler { launchers }
    }

    // FIXME: Add scheduler
    pub fn run(&self) -> Vec<Output> {
        vec![Output::new(
            0,
            String::new(),
            String::new(),
            std::time::Duration::new(5, 0),
        )]
    }
}
