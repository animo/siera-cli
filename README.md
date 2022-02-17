# Aries CLI

> Powered by Animo Solutions

## Summary

The Aries-CLI started out as a small project to practise in Rust. It is now at a point where it can publicly be used and benefit everyone. The goal of this project was to make development easier. Think of things like filling your wallet with test credentials, creating quick invitations and sending basic messages. Many of the functionality is already supported. See [example](#examples) for some useful starter commands.

## Support

At this moment there is only support for some functionality of [**aries-cloudagent-python**](https://github.com/hyperledger/aries-cloudagent-python).
Support for [**aries-framework-javascript**](https://github.com/hyperledger/aries-framework-javascript) will be added in the future via the [rest api](https://github.com/hyperledger/aries-framework-javascript-ext/tree/main/packages/rest).

## Installation

Installig via `cargo` can be done when [rustup](https://www.rust-lang.org/tools/install) is installed.

The tool can be downloaded within a terminal and the correct package manager or via the release section located [here](https://github.com/animo/aries-cli/releases).

### MacOS

**Cargo**

```sh
cargo install aries-cli

```

**Git**

```sh
git clone https://github.com/animo/aries-cli.git
cd aries-cli
cargo install --path .
```

**Brew**

```sh
echo "Coming soon!"
```

### Linux

**Cargo**

```sh
cargo install aries-cli

```

**Git**

```sh
git clone https://github.com/animo/aries-cli.git
cd aries-cli
cargo install --path .
```

### Windows

> Has not been tested extensively

**Cargo**

```sh
cargo install aries-cli

```

**Git**

```sh
git clone https://github.com/animo/aries-cli.git
cd aries-cli
cargo install --path .
```

## Configuration

In order to easily use the aries-cli, you can setup a configuration that allows you to set some default values. the configuration can be initialised with the following command:

```sh
aries-cli init
# or
aries-cli config --initialise
```

This will create a file at `~/.config/aries-cli/config.ini` for \*NIX systems and `TODO` for Windows. It will set a default endpoint to `https://agent.community.animo.id`.

See [here](#extra-documentation) for more information.

## Usage

To see the complete functionality use the `--help` or `-h` flag.
Each individual subcommand also has the `--help` flag, e.g. `aries-cli features --help`.

## Examples

Here are some code examples for common use cases.

### Creating an invitation for the toolbox

```sh
aries-cli -c -s invite -t
```

The `-t` flag makes sure the invite has an alias as `Toolbox`, sets auto accept to `true` and adds `{ "metadata": { "group": "admin" } }` to the body

The `-c` flag copies the output to your clipboard so it can easily be pasted in the toolbox

The `-s` flag suppresses the output to stdout

## Extra documentation

1. [Configuration](./docs/configuration.md)
