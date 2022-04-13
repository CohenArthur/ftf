//! `ExpGot`s are used to represent what was expected, and what was received. If
//! nothing was expected, then any output will be valid. If something was expected, then
//! what was received must match

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExpGot<T: PartialEq> {
    pub(crate) expected: Option<T>,
    pub(crate) got: T,
}

impl<T: PartialEq> ExpGot<T> {
    /// Create a new ExpGot
    pub fn new(expected: Option<T>, got: T) -> ExpGot<T> {
        ExpGot { expected, got }
    }

    /// Check if the content of the ExpGot match
    pub fn eq(&self) -> bool {
        match &self.expected {
            Some(s) => s == &self.got,
            None => true,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ExpString {
    pub(crate) expected: Option<String>,
    pub(crate) got: String,
}

impl ExpString {
    /// Create a new ExpString
    pub fn new(expected: Option<String>, got: String) -> ExpString {
        ExpString { expected, got }
    }

    /// Check if the expected content are contained in the output
    pub fn matches(&self) -> bool {
        match &self.expected {
            Some(msg) => self.got.contains(msg),
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_true() {
        let eg = ExpGot::new(Some("exp_got"), "exp_got");

        assert!(eg.eq())
    }

    #[test]
    fn eq_true_with_none() {
        let eg = ExpGot::new(None, "exp_got");

        assert!(eg.eq())
    }

    #[test]
    fn eq_false() {
        let eg = ExpGot::new(Some("got exp"), "exp_got");

        assert!(!eg.eq())
    }
}
