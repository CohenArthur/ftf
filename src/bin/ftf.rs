use ftf::error::Error;
use ftf::output::{FtOutput, Output};
use ftf::scheduler::Scheduler;
use ftf::yaml::Yaml;
use ftf::INVALID_EXIT;
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long, help = "Files containing the test cases")]
    pub files: Vec<PathBuf>,
    // FIXME: Really add JSON and TOML format
    // FIXME: Switch this to an enum
    #[structopt(short, long, help = "Output format (yaml)")]
    pub output: Option<String>,
    #[structopt(
        short,
        long,
        default_value("1"),
        help = "Amount of threads to use concurrently"
    )]
    pub jobs: usize,
    #[structopt(
        long,
        default_value("%t tests: %p tests passed, %f tests failed"),
        help = "Format string to print results"
    )]
    pub result_fmt: String,
}

impl Args {
    pub fn get_formatter(&self) -> Option<impl Fn(&Output) -> Result<String, Error>> {
        self.output.as_ref().map(|out| match out.as_str() {
            "yaml" => &Yaml::fmt,

            // On invalid strings, use YAML but notify it
            s => {
                eprintln!("Invalid formatter used: {}. Defaulting to YAML", s);
                &Yaml::fmt
            }
        })
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::from_args();
    let scheduler = Scheduler::new(args.files.as_slice(), args.jobs);
    let outputs = scheduler?.run()?;

    let mut retval = 0;

    let (passed, failed) = outputs.iter().fold((0, 0), |(passed, failed), out| {
        if !out.is_valid() {
            retval = INVALID_EXIT;

            if let Some(fmt) = args.get_formatter() {
                match fmt(out) {
                    Ok(out) => eprintln!("{}", out),
                    Err(e) => eprintln!("error when formatting output: {:?}", e),
                }
            }
        }

        match out.exit_code().eq() {
            true => (passed + 1, failed),
            false => (passed, failed + 1),
        }
    });

    let fmt = args.result_fmt;
    let fmt = fmt.replace("%t", &(passed + failed).to_string());
    let fmt = fmt.replace("%p", &passed.to_string());
    let fmt = fmt.replace("%f", &failed.to_string());

    println!("{}", fmt);

    std::process::exit(retval);
}
