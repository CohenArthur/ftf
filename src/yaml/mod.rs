//! The YAML module enables input and output from and to a YAML file.

use crate::input::{FtInput, Input};
use crate::output::{FtOutput, Output};

pub struct Yaml {}

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
            binary: "yaml_parser"
        "#;

        let input = Yaml::parse(data);

        assert_eq!(input.tests[0].name, "YAML test case");
        assert_eq!(input.tests[0].binary, "yaml_parser");
    }

    #[test]
    fn all_fields() {
        let data = r#"
        tests:
          - name: "YAML test case"
            binary: "yaml_parser"
            args:
                - "yml0"
            stdout: "YAML Output"
            stderr: "YAML Errput"
            exit_code: 47
            timeout: 1600
        "#;

        let input = Yaml::parse(data);

        for test_case in input.tests {
            assert_eq!(test_case.stdout, Some(String::from("YAML Output")));
            assert_eq!(test_case.stderr, Some(String::from("YAML Errput")));
            assert_eq!(test_case.exit_code, Some(47));
            assert_eq!(test_case.timeout, Some(1600));
            assert_eq!(test_case.args.unwrap()[0], "yml0");
        }
    }

    #[test]
    fn multiple_cases() {
        let data = r#"
        tests:
          - name: "YAML test case"
            binary: "yaml_parser"

          - name: "Second case"
            binary: "second_yaml"
        "#;

        let input = Yaml::parse(data);

        assert_eq!(input.tests[0].name, "YAML test case");
        assert_eq!(input.tests[0].binary, "yaml_parser");

        assert_eq!(input.tests[1].name, "Second case");
        assert_eq!(input.tests[1].binary, "second_yaml");
    }
}
