---
description: >-
  We hope you'll use the Aries CLI in your day-to-day development work. We're
  trying to make it as intuitive as possible.
---

# Using the Aries CLI

After [installing](installation.md) the Aries CLI, you can run `aries-cli` in your terminal to see all the options available to you. In this guide, we'll show you how to use the Aries CLI in your development process.

> If you'd rather explore functionality through the CLI yourself, just use the `--help` or `-h` flag on any of the subcommands to see more info.

The Aries CLI distinguishes options and subcommands. Options before the subcommands are global, options after the subcommands are specific to that subcommand.

```
aries-cli [OPTIONS] <SUBCOMMAND>
```

Each subcommand has their own options and subcommands, you can see which ones by using the `-h` or `--help` flag at any level.&#x20;



For example, if we wanted to create a schema, a command could look something like:

```
aries-cli --copy schemas create --name "College Degree" --attributes Grade
# Output: schema id
```

Where global option `--copy` copies the output of a command that results in output to your clipboard.&#x20;

But options `--name` and `--attributes` are specific to the subcommand `schemas create`.&#x20;



### Know what to do (--help)

Every subcommand has the option of a -h or --help flag, where you'll see the usage of that subcommand, a description of the options as well as which options are required.&#x20;



/TODO Example aries-cli schemas



/TODO Example aries-cli schemas create



### Know what is happening (--verbosity)

We believe that good logging makes for a good CLI user experience, but we also believe a productivity focused tool should not flood the terminal with logging. That is why the Aries CLI uses different log-levels.

| CLI flags                                 | Rust logger | Description                           |
| ----------------------------------------- | ----------- | ------------------------------------- |
| `--verbose` or `-v`                       | `info!`     | up to **informational**-level logging |
| `--verbose --verbose` or `-vv`            | `debug!`    | up to **debug**-level logging         |
| `--verbose --verbose --verbose` or `-vvv` | `trace!`    | up to **trace**-level logging         |

This means **by default** we will only log command output, warnings and errors. If you're looking for more info, use the more extensive verbosity levels.&#x20;

****

