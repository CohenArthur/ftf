mod args;
mod input;
mod launcher;
mod output;
mod scheduler;

mod yaml;

use args::FtArgs;
use scheduler::Scheduler;

fn main() {
    let args = FtArgs::collect();

    let scheduler = Scheduler::from_args(&args);

    // FIXME: Add output
}
