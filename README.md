# Aries CLI

A tool that allows you to interact with an Aries Cloudagent.

Made by Animo Solutions

## start

```bash
cargo run -- <COMMAND>

# For example, for testing

cargo run -- -e=XXX invite --qr
```

## Info

The binary that is build via `cargo build` is called: `acl`.

## usage

```bash

git clone https://github.com/animo/aries-cli

cd aries-cli

cargo install --path .

# Now the binary, named `acl` is installed in your path

acl --help

acl invite --help
```
