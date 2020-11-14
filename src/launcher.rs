//! The Launcher module is the one responsible for executing the different
//! test cases and reporting the results

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use wait_timeout::ChildExt;

use crate::exp_got::ExpGot;
use crate::output::{Output, INVALID_EXIT};

static MAX_SECS: u64 = u64::MAX;

pub struct Launcher {
    name: String,
    binary: String,
    args: Option<Vec<String>>,

    stdout: Option<String>,
    stderr: Option<String>,
    exit_code: Option<i32>,
    timeout: Option<Duration>,
}

impl Launcher {
    pub fn new(
        name: String,
        binary: String,
        args: Option<Vec<String>>,
        stdout: Option<String>,
        stderr: Option<String>,
        exit_code: Option<i32>,
        timeout: Option<Duration>,
    ) -> Launcher {
        Launcher {
            name,
            binary,
            args,
            stdout,
            stderr,
            exit_code,
            timeout,
        }
    }

    fn binary(&self) -> &str {
        &self.binary
    }

    fn args(&self) -> Option<&Vec<String>> {
        self.args.as_ref()
    }

    pub fn run(&self) -> Result<Output, std::io::Error> {
        let start = Instant::now();

        let mut child = Command::new(self.binary())
            .args(self.args().unwrap_or(&vec![]))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let status_code =
            match child.wait_timeout(self.timeout.unwrap_or(Duration::from_secs(MAX_SECS)))? {
                Some(status) => status.code(),
                None => {
                    child.kill()?;
                    child.wait()?.code()
                }
            };

        let (mut out, mut err) = (String::new(), String::new());

        child.stdout.unwrap().read_to_string(&mut out)?;
        child.stderr.unwrap().read_to_string(&mut err)?;

        // FIXME: No clone
        Ok(Output::new(
            self.name.clone(),
            ExpGot::new(self.exit_code, status_code.unwrap_or(INVALID_EXIT)),
            ExpGot::new(self.stdout.clone(), out),
            ExpGot::new(self.stderr.clone(), err),
            ExpGot::new(self.timeout, start.elapsed()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo() {
        let l = Launcher::new(
            "echo".to_owned(),
            "echo".to_owned(),
            Some(vec!["hello".to_owned()]),
            None,
            None,
            None,
            None,
        );
        let o = l.run().unwrap();

        assert_eq!(o.exit_code(), 0);
        assert_eq!(o.out(), "hello\n");
        assert_eq!(o.err(), "");
    }

    #[test]
    fn sleep_2() {
        let l = Launcher::new(
            "sleep".to_owned(),
            "sleep".to_owned(),
            Some(vec!["2".to_owned()]),
            None,
            None,
            None,
            None,
        );
        let o = l.run().unwrap();

        assert_eq!(o.exit_code(), 0);
        assert_eq!(o.time().as_secs(), 2);
    }

    #[test]
    fn sleep_with_shorter_timeout() {
        let l = Launcher::new(
            "sleep".to_owned(),
            "sleep".to_owned(),
            Some(vec!["2".to_owned()]),
            None,
            None,
            None,
            Some(Duration::from_secs(1)),
        );
        let o = l.run().unwrap();

        assert_ne!(o.exit_code(), 0);
        assert_eq!(o.time().as_secs(), 1);
    }
}
