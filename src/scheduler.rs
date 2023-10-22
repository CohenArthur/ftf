//! The scheduler handles a set of launchers, deciding who to launch and who
//! to wait for.

use crate::error::Error;
use crate::yaml::Yaml;

use crate::input::{Input, Inputter};
use crate::launcher::Launcher;
use crate::output::Output;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

pub struct Context<'ctx> {
    files: &'ctx [PathBuf],
    threads: usize,
    extra_args: &'ctx [String],
}

impl<'ctx> Context<'ctx> {
    pub fn new(
        files: &'ctx [PathBuf],
        threads: usize,
        extra_args: &'ctx [String],
    ) -> Context<'ctx> {
        Context {
            files,
            threads,
            extra_args,
        }
    }
}

pub struct Scheduler {
    threads: usize,
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

    pub fn new(ctx: Context<'_>) -> Result<Scheduler, Error> {
        let mut launchers = Vec::new();

        for file in ctx.files {
            let input = Scheduler::dispatch_file(file)?;

            for test_case in input.tests {
                // If there are no arguments, pass an empty vector
                let mut args = test_case.args.unwrap_or_default();
                args.append(&mut ctx.extra_args.to_vec());

                launchers.push(Launcher::new(
                    test_case.name,
                    test_case.binary,
                    args,
                    test_case.stdout,
                    test_case.stderr,
                    test_case.exit_code.map(|v| v as i32),
                    test_case.timeout.map(Duration::from_secs),
                ));
            }
        }

        Ok(Scheduler {
            threads: ctx.threads,
            launchers,
        })
    }

    pub fn run(self) -> Result<Vec<Output>, Error> {
        let pool = ThreadPoolBuilder::new().num_threads(self.threads).build()?;

        pool.install(move || {
            self.launchers
                .into_par_iter()
                .map(|launcher| launcher.run())
                .collect()
        })
    }
}
