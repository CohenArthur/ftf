//! The assert module makes sure that the output matches the given input. In
//! case it doesn't, it produces a detailed report

use crate::input::TestCase;
use crate::output::Output;

fn handle_difference(input: &TestCase, output: &Output) {
}

// FIXME: Add detailed report
pub fn check(input: &TestCase, output: &Output) -> bool {
    let mut retval = match input.exit_code {
        Some(exit_code) => exit_code == output.exit_code,
        None => output.exit_code == 0
    };

    retval = if retval {
        match &input.stdout {
            Some(stdout) => stdout == &output.stdout,
            None => retval
        }
    } else {
        retval
    };

    retval = if retval {
        match &input.stderr {
            Some(stderr) => stderr == &output.stderr,
            None => retval
        }
    } else {
        retval
    };

    retval
}
