mod args;

use args::Args;

use anyhow::Result;
use colored::Colorize;
use ftf::Scheduler;

fn main() -> Result<()> {
    let args = Args::collect();

    let scheduler = Scheduler::from_files(&args.files);
    let outputs = scheduler.run();

    let mut retval = 0;
    let mut passed = 0;
    let mut failed = 0;

    let formatter = ftf::get_formatter(&args.output)?;

    outputs.iter().for_each(|o| {
        o.display(&formatter, &mut retval);

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
