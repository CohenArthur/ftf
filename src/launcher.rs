//! The Launcher module is the one responsible for executing the different
//! test cases and reporting the results

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use wait_timeout::ChildExt;

use crate::output::{Output, INVALID_EXIT};

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

    pub fn run(&self) -> Result<Output, std::io::Error> {
        let start = Instant::now();

        let mut child = Command::new(self.binary())
            .args(self.args().unwrap_or(&vec![]))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let max_duration = Duration::from_secs(MAX_SECS);

        let status_code = match child.wait_timeout(self.timeout.unwrap_or(max_duration))? {
            Some(status) => status.code(),
            None => {
                child.kill()?;
                child.wait()?.code()
            }
        };

        let (mut out, mut err) = (String::new(), String::new());

        // FIXME: No unwrap
        child.stdout.unwrap().read_to_string(&mut out)?;
        child.stderr.unwrap().read_to_string(&mut err)?;

        Ok(Output::new(
            status_code.unwrap_or(INVALID_EXIT),
            out,
            err,
            start.elapsed(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo() {
        let l = Launcher::new("echo".to_owned(), Some(vec!["hello".to_owned()]), None);
        let o = l.run().unwrap();

        assert_eq!(o.exit_code(), 0);
        assert_eq!(o.out(), "hello\n");
        assert_eq!(o.err(), "");
    }

    #[test]
    fn sleep_2() {
        let l = Launcher::new("sleep".to_owned(), Some(vec!["2".to_owned()]), None);
        let o = l.run().unwrap();

        assert_eq!(o.exit_code(), 0);
        assert_eq!(o.time().as_secs(), 2);
    }

    #[test]
    fn sleep_with_shorter_timeout() {
        let l = Launcher::new(
            "sleep".to_owned(),
            Some(vec!["2".to_owned()]),
            Some(Duration::from_secs(1)),
        );
        let o = l.run().unwrap();

        assert_ne!(o.exit_code(), 0);
        assert_eq!(o.time().as_secs(), 1);
    }

    #[test]
    fn piped() {
        let l = Launcher::new(
            "echo".to_owned(),
            Some(
                "hello | wc -l"
                    .to_owned()
                    .split(' ')
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>(),
            ),
            None,
        );
        let o = l.run().unwrap();

        assert_ne!(o.exit_code(), 0);
        assert_ne!(o.out(), "1");
    }
}
