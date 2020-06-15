//! The Launcher module is the one responsible for executing the different
//! test cases and reporting the results

use crate::output::Output;

use std::time::Duration;

pub struct Launcher {
    binary: String,
    args: Option<Vec<String>>,
    timeout: Duration,
}

impl Launcher {
    pub fn new(binary: String, args: Option<Vec<String>>, timeout: Duration) -> Launcher {
        Launcher { binary, args, timeout }
    }

    pub fn run() -> Output {
        Output::new(0, String::new(), String::new(), Duration::new(5, 0)) // FIXME: Add logic
    }
}
