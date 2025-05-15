# `sorn_cli`

`scorn_cli` is a CLI tool to generate <REPEAT> random strings of length <LENGTH> and sleeps for <SLEEP> seconds between outputs.

The program is designed to be run from the command line and takes the following arguments:

- `--length`: The length of the random string to generate.

- `--sleep`: The number of seconds to sleep between generating random strings.

- `--repeat`: The number of times to repeat the process of generating a random string and sleeping.

The program uses the `clap` crate for command line argument parsing. The program uses the `rand` crate to generate random strings and the `std::thread` module to sleep.

```verbatim
Usage: sorn_cli [OPTIONS]

Options:
  -l, --length <LENGTH>
          number of random characters to generate
          
          [default: 32]

  -s, --sleep <SLEEP>
          number of seconds to sleep
          
          [default: 10]

  -r, --repeat <REPEAT>
          number of times to repeat
          
          [default: 1000000000]

  -h, --help
          Print help (see a summary with '-h')
```

## Building

- Update dependencies

```bash
cargo update
```

- Check with `clippy`:

```bash
cargo clippy -- -W clippy::pedantic
```

- Run current version of code and print help

```bash
cargo run -- --help
```

- Install on your local computer

```bash
cargo install --path .
```
