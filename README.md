# Basic rust echo application

This is an implementation of `echo` with a minimal feature set.

## Build

Use Cargo to build the application.

```shell
cargo build [--release]
```

## Usage

```shell
echo [-n] [args...]
```

Prints `args` to standard output, terminating with a newline.

The `-n` flag causes the final newline to be omitted.
