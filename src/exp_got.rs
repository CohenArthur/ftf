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
        match &self.expected {
            Some(s) => s == &self.got,
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
