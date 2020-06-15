//! The YAML module enables input and output from and to a YAML file.

use crate::input::{Input, FtInput};
use crate::output::{Output, FtOutput};

pub struct Yaml {
}

impl FtInput for Yaml {
    fn parse(data: String) -> Input {
        serde_yaml::from_str(&data).unwrap()
    }
}

impl FtOutput for Yaml {
    fn fmt(data: &Output) -> String {
        serde_yaml::to_string(data).unwrap()
    }
}
