//! Logging module for ftf. This is just a few basic functions in order to
//! display success, failures, or that a test is currently running

use colored::Colorize;
use std::io::{stdout, Write};

use crate::error::Error;

pub fn running(test_name: &'_ str) -> Result<(), Error> {
    print!("{} {}", "*".yellow().blink(), test_name);
    stdout().lock().flush()?;

    Ok(())
}

pub fn success(test_name: &'_ str) {
    println!("\r[{}] {}", "OK".green(), test_name);
}

pub fn failure(test_name: &'_ str) {
    println!("\r[{}] {}", "KO".red(), test_name);
}
