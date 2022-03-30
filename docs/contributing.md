# Contributing

We welcome contributions that make the CLI better. If you do not have a specific
contribution in mind look for issues labelled as `good first issue`.

## Fixes

If you have a fix, go straight to "Getting set up".

## Features

If you have something bigger in mind (structural changes, new features) we strongly
encourage you to create a new GitHub issue with:

* motivation for the changes you want to make
* approach for how you want to make them

This gives your new idea the best chance to get accepted by the
maintainers.

## Getting set up

```
git clone https://github.com/animo/aries-cli.git
```

Once you have the code locally you can run commands using:

```sh
# "cargo run -q --" instead of aries-cli
cargo run -q -- <cmd>
```

### Tests

We love tests, but recognize that there is a shortage of them at the moment. Take a look at
[Rust's guide](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) on how to create an automated test. We are happy
to provide support for writing tests on your PR.

A simple suite of tests can be executed by running `./tests/run.sh`.


### Using log levels

We believe that good logging makes for a good CLI user experience. However,
a productivity focussed tool should not flood the terminal with logs. That is why
we have different log-levels:

| CLI flags | Rust logger | Description |
| --------- | ----------- | ----------- |
| `--verbose` or `-v` | `info!` | up to **informational**-level logging |
| `--verbose --verbose` or `-vv` | `debug!` | up to **debug**-level logging |
| `--verbose --verbose --verbose` or `-vvv` | `trace!` | up to **trace**-level logging |

This means **by default** we will only log command output, warnings and errors.

**Note**: for command output, we use `println!` to ensure that the result of a command
returned on stdout. This implies output printed via `println!` should be machine readable a
few exceptions.
