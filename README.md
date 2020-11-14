# ft

Parallel Functional Testing framework, written in Rust

# Why ?

`ft` provides an alternative to constantly rewriting the same functional test
framework for every project you start. It is written in pure Rust, and allows
parallel testing, as well as an important set of input/output formats.

# How ?

Look at the [syntax](SYNTAX.md) to get started with creating your own tests.
Then, simply run `ft` on the files you created.

# Features

* [ ] Parallel execution
* [x] Timeout
* [x] Assert on `stdout`
* [x] Assert on `stderr`
* [x] Assert on exit-code
* [x] Input from YAML
* [ ] Input from JSON
* [ ] Input from TOML
* [x] YAML output
* [ ] JSON output
* [ ] TOML output
