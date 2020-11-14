mod args;
mod exp_got;
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
    let outputs = scheduler.run();

    let mut retval = 0;

    outputs.iter().for_each(|o| o.display(&args, &mut retval));

    std::process::exit(retval);
}
