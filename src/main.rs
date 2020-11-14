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

    let l = crate::launcher::Launcher::new("echo".to_owned(), Some(vec!["hello".to_owned()]), None);
    let o = l.run();

    dbg!(o);

    // FIXME: Add output
}
