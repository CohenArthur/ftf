mod args;
mod error;
mod exp_got;
mod input;
mod launcher;
mod output;
mod scheduler;

mod yaml;

use colored::Colorize;

use args::Args;
use scheduler::Scheduler;

fn main() -> anyhow::Result<()> {
    let args = Args::collect();

    let scheduler = Scheduler::from_args(&args);
    let outputs = scheduler?.run()?;

    let mut retval = 0;

    let (passed, failed) = outputs.iter().fold((0, 0), |(passed, failed), out| {
        out.check_error(&args, &mut retval);

        match out.exit_code().eq() {
            true => (passed + 1, failed),
            false => (passed, failed + 1),
        }
    });

    println!(
        "{} tests: {} tests passed, {} tests failed",
        passed + failed,
        passed.to_string().green(),
        failed.to_string().red()
    );

    std::process::exit(retval);
}
