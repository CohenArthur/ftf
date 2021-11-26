mod error;
mod exp_got;
mod input;
mod launcher;
mod output;
mod scheduler;

mod yaml;

pub use error::Error;
pub use launcher::Launcher;
pub use output::Output;
pub use scheduler::Scheduler;

use output::FtOutput;
use yaml::Yaml;

type Formatter = dyn Fn(&output::Output) -> String;

pub fn get_formatter(
    fmt_string: &Option<String>,
) -> Result<&'static dyn Fn(&output::Output) -> String, Error> {
    match fmt_string.as_ref() {
        Some(s) => match s.as_str() {
            "yaml" => Ok(&Yaml::fmt),
            s => Err(Error::InvalidFormatter(s.to_owned())),
        },
        // Default formatter is YAML
        None => Ok(&Yaml::fmt),
    }
}
