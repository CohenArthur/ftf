//! The Launcher module is the one responsible for executing the different
//! test cases and reporting the results

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use wait_timeout::ChildExt;

use crate::error::Error;
use crate::expected::{ExpGot, ExpString};
use crate::log;
use crate::output::Output;
use crate::INVALID_EXIT;

static MAX_SECS: u64 = u64::MAX;

pub struct Launcher {
    name: String,
    binary: String,
    args: Vec<String>,

    stdout: Option<String>,
    stderr: Option<String>,
    exit_code: Option<i32>,
    timeout: Option<Duration>,
}

impl Launcher {
    pub fn new(
        name: String,
        binary: String,
        args: Vec<String>,
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

    fn args(&self) -> &Vec<String> {
        self.args.as_ref()
    }

    pub fn run(self) -> Result<Output, Error> {
        let start = Instant::now();

        log::running(&self.name)?;

        let mut child = Command::new(self.binary())
            .args(self.args())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let status_code = match child.wait_timeout(
            self.timeout
                .unwrap_or_else(|| Duration::from_secs(MAX_SECS)),
        )? {
            Some(status) => status.code(),
            None => {
                child.kill()?;
                child.wait()?.code()
            }
        };

        let (mut out, mut err) = (String::new(), String::new());

        if let Some(mut stdout) = child.stdout {
            if stdout.read_to_string(&mut out).is_err() {
                out.push_str("stdout does not contain valid UTF-8 - refusing to handle it");
            }
        }
        if let Some(mut stderr) = child.stderr {
            if stderr.read_to_string(&mut err).is_err() {
                out.push_str("stderr does not contain valid UTF-8 - refusing to handle it");
            }
        }

        Ok(Output::new(
            self.name,
            ExpGot::new(
                Some(self.exit_code.unwrap_or(0)),
                status_code.unwrap_or(INVALID_EXIT),
            ),
            ExpString::new(self.stdout, out),
            ExpString::new(self.stderr, err),
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
            vec!["hello".to_owned()],
            None,
            None,
            None,
            None,
        );
        let o = l.run().unwrap();

        assert_eq!(o.exit_code_got(), 0);
        assert_eq!(o.out(), "hello\n");
        assert_eq!(o.err(), "");
    }

    #[test]
    fn sleep_2() {
        let l = Launcher::new(
            "sleep".to_owned(),
            "sleep".to_owned(),
            vec!["2".to_owned()],
            None,
            None,
            None,
            None,
        );
        let o = l.run().unwrap();

        assert_eq!(o.exit_code_got(), 0);
        assert_eq!(o.time().as_secs(), 2);
    }

    #[test]
    fn sleep_with_shorter_timeout() {
        let l = Launcher::new(
            "sleep".to_owned(),
            "sleep".to_owned(),
            vec!["2".to_owned()],
            None,
            None,
            None,
            Some(Duration::from_secs(1)),
        );
        let o = l.run().unwrap();

        assert_ne!(o.exit_code_got(), 0);
        assert!(o.time().as_secs() <= 1);
    }
}
