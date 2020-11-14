//FIXME: Add documentation

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExpGot<T> {
    pub expected: Option<T>,
    pub got: T,
}

impl<T> ExpGot<T> {
    pub fn new(expected: Option<T>, got: T) -> ExpGot<T> {
        ExpGot { expected, got }
    }
}
