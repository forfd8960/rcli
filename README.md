# RCli

**Rust Cli Tool✌️** ✌️

## Errors

### cargo deny check failed

```
[ERROR] failed to open advisory database: "$HOME/.cargo/advisory-db/github.com-2f857891b7f43c59" does not appear to be a git repository: Could not retrieve metadata of "$HOME/.cargo/advisory-db/github.com-2f857891b7f43c59": No such file or directory (os error 2)
```

fix by adding:

refer: https://github.com/EmbarkStudios/cargo-deny/pull/420

```toml
[advisories]
git-fetch-with-cli = true
```

## CSV

### install dependency

```sh
cargo add clap --features derive

cargo add csv

cargo add serde --features derive

cargo add anyhow

cargo add serde-json
```

```sh
Usage: rcli <COMMAND>

Commands:
  csv   convert csv to json
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version


❯ cargo run -- csv -i test.csv
warning: unused manifest key: package.author
   Compiling rcli v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/rcli csv -i test.csv`
Opts { cmd: Csv(CsvOpts { input: "test.csv", output: "output.json", delimiter: ',', header: true }) }
```

### Value parser

```sh
❯ cargo run -- csv -i test.csv
warning: unused manifest key: package.author
   Compiling rcli v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/rcli csv -i test.csv`
error: invalid value 'test.csv' for '--input <INPUT>': File is not exists
```
