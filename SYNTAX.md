# `ft` syntax

An input file should respect the following structure:

```yaml
tests:
    - name: "Test name"
      binary: "binary to test"
      args:
        - "arg1"
        - "arg2"
        - "..."
     # Other options
```

Here is a list of options that `ft` can handle:

|Option name|Type|Description|Default value|
|---|---|---|---|
|`name`|String|Name of the test|None|
|`binary`|String|Binary to test|None|
|`args`|Array of Strings|Arguments to give to the tested binary|None|
|`stdout`|String|What the test case should output on `stdout`|None|
|`stderr`|String|What the test case should errput on `stderr`|None|
|`exit_code`|Integer|shell exit code of the test case|0|
|`timeout`|Integer|Maximum allotted time to the test case, in seconds|1000|

## Arguments

To pass arguments to a binary, do as you would when writing the command
yourself. This means that in order to do the following:

`binary arg0 arg1 -o opt0`

You would need to create the following YAML

```yaml
tests:
    - name: "..."
      binary: "binary"
      args:
        - "arg0"
        - "arg1"
        - "-o"
        - "opt0"
```
