//! FtArgs handle argument parsing for the `ft` binary

use crate::error::Error;
use crate::output::{FtOutput, Output};
use crate::yaml::Yaml;

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long, help = "Files containing the test cases")]
    pub files: Vec<PathBuf>,

    // FIXME: Really add JSON and TOML format
    #[structopt(short, long, help = "Output format (yaml)")]
    pub output: Option<String>,

    #[structopt(
        short,
        long,
        default_value("1"),
        help = "Amount of threads to use concurrently"
    )]
    pub jobs: usize,
}

impl Args {
    pub fn collect() -> Args {
        Args::from_args()
    }

    pub fn get_formatter(&self) -> Option<impl Fn(&Output) -> Result<String, Error>> {
        self.output.as_ref().map(|out| match out.as_str() {
            "yaml" => &Yaml::fmt,

            // On invalid strings, use YAML but notify it
            s => {
                eprintln!("Invalid formatter used: {}. Defaulting to YAML", s);
                &Yaml::fmt
            }
        })
    }
}
