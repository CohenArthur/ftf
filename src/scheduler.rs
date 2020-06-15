//! The scheduler handles a set of launchers, deciding who to launch and who
//! to wait for.

use crate::yaml::Yaml;

use crate::input::{Input, FtInput};
use crate::output::Output;
use crate::launcher::Launcher;
use crate::args::FtArgs;

use std::path::PathBuf;
use std::fs;

pub struct Scheduler {
    launchers: Vec<Launcher>,
}

impl Scheduler {
    fn dispatch_file(file: &PathBuf) -> Input {
        // FIXME: Cleanup
        match file.extension().unwrap().to_str() {
            Some("yml") => Yaml::parse(&fs::read_to_string(file).unwrap()),
            None => panic!("Not a valid filename {}", file.to_str().unwrap())
        }
    }

    pub fn from_args(args: &FtArgs) -> Scheduler {
        let mut input_vec = Vec::new();

        for file in args.files {
            input_vec.push(dispatch_file(file));
        }

    }

    pub fn run() -> Vec<Output> {
        vec!(Output::new(0, String::new(), String::new(), std::time::Duration::new(5, 0)))
    }
}
