//! The YAML module enables input and output from and to a YAML file.

use crate::input::{Input, FtInput};
use crate::output::{Output, FtOutput};

pub struct Yaml {
}

impl FtInput for Yaml {
    fn parse(data: &str) -> Input {
        serde_yaml::from_str(&data).unwrap()
    }
}

impl FtOutput for Yaml {
    fn fmt(data: &Output) -> String {
        serde_yaml::to_string(data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_input() {
        let data = r#"
        tests:
          - name: "YAML test case"
            cmd: "yaml_parser"
        "#;

        let input = Yaml::parse(data);

        assert_eq!(input.tests[0].name, "YAML test case");
        assert_eq!(input.tests[0].cmd, "yaml_parser");
    }

    #[test]
    fn all_fields() {
        let data = r#"
        tests:
          - name: "YAML test case"
            cmd: "yaml_parser"
            args:
                - "yml0"
                - "yml1"
                - "yml2"
            stdout: "YAML Output"
            stderr: "YAML Errput"
            exit_code: 47
            timeout: 1600
        "#;

        let input = Yaml::parse(data);

        assert_eq!(input.tests[0].stdout, Some(String::from("YAML Output")));
        assert_eq!(input.tests[0].stderr, Some(String::from("YAML Errput")));
        assert_eq!(input.tests[0].exit_code, Some(47));
        assert_eq!(input.tests[0].timeout, Some(1600));
    }
}
