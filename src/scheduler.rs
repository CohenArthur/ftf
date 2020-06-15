//! The scheduler handles a set of launchers, deciding who to launch and who
//! to wait for.

use crate::launcher::Launcher;
use crate::output::Output;

pub struct Scheduler {
    launchers: Vec<Launcher>,
}

impl Scheduler {
    pub fn new(launchers: Vec<Launcher>) -> Scheduler {
        Scheduler {
            launchers,
        }
    }

    pub fn run() -> Vec<Output> {
        vec!(Output::new(0, String::new(), String::new(), std::time::Duration::new(5, 0)))
    }
}
