//! FtArgs handle argument parsing for the `ft` binary

use crate::output::{FtOutput, Output};
use crate::yaml::Yaml;

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long, help = "Files containing the test cases")]
    pub files: Vec<PathBuf>,

    // FIXME: Really add JSON and TOML format
    #[structopt(short, long, help = "Output format (yaml, json, toml)")]
    pub output: Option<String>,
}

impl Args {
    pub fn collect() -> Args {
        Args::from_args()
    }

    pub fn get_formatter(&self) -> &dyn Fn(&Output) -> String {
        match self.output.as_ref() {
            Some(s) => match s.as_str() {
                "yaml" => &Yaml::fmt,

                // On invalid strings, use YAML but notify it
                s => {
                    eprintln!("Invalid formatter used: {}. Using YAML", s);
                    &Yaml::fmt
                }
            },

            // Default formatter is YAML
            None => &Yaml::fmt,
        }
    }
}
