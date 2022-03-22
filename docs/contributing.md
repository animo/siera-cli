# Contributing

We welcome contributions that to make the CLI better. If you do not have a specific
contribution in mind we encourage you to look for issues labelled with as
`good first issue` or look for missing actions in our [support matrix](./support_matrix.md).

## Fixes

If you have a fix, go straight to "Getting set up".

## Features

If you have something bigger in mind (structural changes, new features) we strongly
encourage you to create a new GitHub issue that documents:

* motivate the changes you want to make
* how you want to make them

This gives your new idea the best chance to get accepted by the repository
maintainers.

## Getting set up

```
git clone https://github.com/animo/aries-cli.git
```

Once you have the code locally, you should be able to test out the current
source code by running:

```sh
# "cargo run -q --" instead of aries-cli
cargo run -q -- <cmd>
```

### Tests

We love tests, but recognize that there is a shortage of them at the moment. We
encourage you to take a look at [Rust's guide](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) on how to create an automated test. We are happy
to provide support for writing tests on the PR.

Currently a simple suite of tests can be executed by running `./tests/run.sh`.


### Using log levels

We believe that good logging makes for a good CLI user experience but we also believe
a productivity focussed tool should not flood the terminal with logging. That is why
we have built in different log-levels:

| CLI flags | Rust logger | Description |
| --------- | ----------- | ----------- |
| `--verbose` or `-v` | `info!` | up to **informational**-level logging |
| `--verbose --verbose` or `-vv` | `debug!` | up to **debug**-level logging |
| `--verbose --verbose --verbose` or `-vvv` | `trace!` | up to **trace**-level logging |

This means **by default** we will only log command output, warnings and errors.

**Note**: for command output, we use `println!` to ensure that the result of a command
returned on stdout.
