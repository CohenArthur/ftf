use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct FtArgs {
    #[structopt(short, long, help = "Files containing the test cases")]
    pub files: Vec<PathBuf>,

    // FIXME: Really add JSON and TOML format
    #[structopt(short, long, help = "Output format (yaml, json, toml)")]
    pub output: String,
}

impl FtArgs {
    pub fn collect() -> FtArgs {
        FtArgs::from_args()
    }
}
