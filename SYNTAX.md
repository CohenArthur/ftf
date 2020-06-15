# `ft` syntax

An input file should respect the following structure:

```yaml
tests:
    - name: "Test name"
      cmd: "binary to test"
      args:
        - "arg1"
        - "arg2"
        - "..."
     # Other options
```

Here is a list of options that `ft` can handle:

|Option name|Type|Description|
|---|---|---|
|`stdout`|String|What the test case should output on `stdout`|
|`stderr`|String|What the test case should errput on `stderr`|
|`exit_code`|Integer|shell exit code of the test case|
|`timeout`|Integer|Maximum allotted time to the test case, in seconds|
