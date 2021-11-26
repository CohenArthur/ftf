//! FtArgs handle argument parsing for the `ft` binary

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
}
