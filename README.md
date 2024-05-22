## RUST YAML LINTER

## Description
This is a simple linter for yaml files written in Rust. It checks for the following:
- Basic yaml structure
- List sorting

There are two binaries in this project:
- `main` which is the main linter
- `generate` which creates yaml test data

## Usage

These are developer instructions for use with Cargo (Rust's package manager).

Using `--` is necessary to pass arguments to the binary.

### Linter
The linter can be run with the following command:

```bash
cargo run --bin main
```
this is the default mode of working and will validate all yaml files it finds recursively
and sort all lists.

Optional flags can be provided such as `--verbose` to include more information in the output or
`--check` to only check the yaml files without sorting them.

e.g. 
```bash
cargo run --bin main -- --check
```

### Test Data Generator
The test data generator can be run with the following command:

```bash
cargo run --bin generate
```

This will create an `output` directory with a number of yaml files in it.

The number of files and the number of entries in each file can be controlled with the `--files` and `--depth` flags respectively.
