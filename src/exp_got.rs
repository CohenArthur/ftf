//FIXME: Add documentation

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExpGot<T: PartialEq> {
    pub expected: Option<T>,
    pub got: T,
}

impl<T: PartialEq> ExpGot<T> {
    pub fn new(expected: Option<T>, got: T) -> ExpGot<T> {
        ExpGot { expected, got }
    }

    pub fn eq(&self) -> bool {
        match &eg.expected {
            Some(s) => s == &eg.got,
            None => true,
        }
    }
}
