mod args;
mod error;
mod exp_got;
mod input;
mod launcher;
mod output;
mod scheduler;

mod yaml;

use colored::Colorize;
use anyhow::Result;

use args::FtArgs;
use scheduler::Scheduler;

fn main() -> Result<()> {
    let args = FtArgs::collect();

    let scheduler = Scheduler::from_args(&args)?;
    let outputs = scheduler.run();

    let mut retval = 0;
    let mut passed = 0;
    let mut failed = 0;

    outputs.iter().for_each(|o| {
        o.display(&args, &mut retval);

        match o.exit_code().eq() {
            true => passed += 1,
            false => failed += 1,
        }
    });

    println!(
        "{} tests passed, {} tests failed",
        passed.to_string().green(),
        failed.to_string().red()
    );

    std::process::exit(retval);
}
