//! The Launcher module is the one responsible for executing the different
//! test cases and reporting the results

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::io::Read;

use wait_timeout::ChildExt;

use crate::output::Output;

static MAX_SECS: u64 = u64::MAX;

pub struct Launcher {
    binary: String,
    args: Option<Vec<String>>,
    timeout: Option<Duration>,
}

impl Launcher {
    pub fn new(binary: String, args: Option<Vec<String>>, timeout: Option<Duration>) -> Launcher {
        Launcher {
            binary,
            args,
            timeout,
        }
    }

    fn binary(&self) -> &str {
        &self.binary
    }

    fn args(&self) -> Option<&Vec<String>> {
        self.args.as_ref()
    }

    pub fn run(&self) -> Output {
        let start = Instant::now();

        // FIXME: Don't unwrap
        let mut child = Command::new(self.binary())
            .args(self.args().unwrap_or(&vec![]))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let max_duration = Duration::from_secs(MAX_SECS);

        // FIXME: Don't unwrap
        let status_code = match child
            .wait_timeout(self.timeout.unwrap_or(max_duration))
            .unwrap()
        {
            Some(status) => status.code(),
            None => {
                child.kill().unwrap();
                child.wait().unwrap().code()
            }
        };

        let (mut out, mut err) = (String::new(), String::new());

        // FIXME: No unwrap
        child.stdout.unwrap().read_to_string(&mut out).unwrap();
        child.stderr.unwrap().read_to_string(&mut err).unwrap();

        Output::new(
            status_code.unwrap(),
            out,
            err,
            start.elapsed(), // FIXME: Check if time is valid
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo() {
        let l = Launcher::new("echo".to_owned(), Some(vec!["hello".to_owned()]), None);
        let o = l.run();

        assert_eq!(o.exit_code(), 0);
        assert_eq!(o.out(), "hello\n");
        assert_eq!(o.err(), "");
    }
 
    #[test]
    fn sleep_2() {
        let l = Launcher::new("sleep".to_owned(), Some(vec!["2".to_owned()]), None);
        let o = l.run();

        assert_eq!(o.exit_code(), 0);
        assert_eq!(o.time().as_secs(), 2);
    }
}
