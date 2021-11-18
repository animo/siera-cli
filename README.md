# Aries Data Generator Toolkit

A tool to interact with Aries instances for quick data manipulation.

Made by Animo Solutions

## start

```bash
cargo run <COMMAND>

# For example, for testing

cargo run run
```

## Info

The binary that is build via `cargo build` is called: 'inu'. The code name for this project.

## usage

the repository has a default json configured

```bash

git clone https://github.com/animo/aries-data-generator-toolkit.github

cd aries-data-generator-toolkit

cargo install --path .

# Now the binary, named `inu` is installed in your path

inu agent -i -c <PATH_TO_JSON>

# with default json (called from within the repository)

inu agent -i
```
