use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct FtArgs {
    #[structopt(short, long, help = "Files containing the test cases")]
    files: Vec<PathBuf>,
}

impl FtArgs {
    pub fn collect() -> FtArgs {
        FtArgs::from_args()
    }
}
