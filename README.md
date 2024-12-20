<h1 align="center">Rusty TODO</h1>

## Get started

### Build
`cargo build`

### Tests
`cargo test`

## Usage

### Release
`cargo build --release`
> 🎈 This command will release a binary optimized

### Help with subcommands
`./target/release/todo --help`

```
Todo 0.1.0
A command line to-do app written in Rust

USAGE:
    todo <pathbuf> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <pathbuf>

SUBCOMMANDS:
    add
    done
    help    Prints this message or the help of the given subcommand(s)
    list
```
