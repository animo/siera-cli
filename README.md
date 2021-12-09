# Aries Cloudagent Controller Ferris

A tool that allows you, via Ferris, to interact with an Aries Cloudagent.

Made by Animo Solutions

## start

```bash
cargo run -- <COMMAND>

# For example, for testing

cargo run -- -e=XXX invite --qr
```

## Info

The binary that is build via `cargo build` is called: 'accf'.

## usage

```bash

git clone https://github.com/animo/aries-data-generator-toolkit

cd aries-data-generator-toolkit

cargo install --path .

# Now the binary, named `accf` is installed in your path

accf --help

accf invite --help
```
