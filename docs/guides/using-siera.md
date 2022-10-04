---
description: Siera follows CLI tool conventions to make it easy to use.
---

# Using Siera

After [installing](installation.md) Siera, you can run `siera` in your terminal to see all the options available to you. In this guide, we'll show you how to use Siera in your development process.

> If you'd rather explore functionality through the CLI yourself, just use the `--help` or `-h` flag on any of the subcommands to see more info.

Siera distinguishes options and subcommands. Options before the subcommands are global, options after the subcommands are specific to that subcommand.

```
siera [OPTIONS] <SUBCOMMAND>
```

Each subcommand has their own options and subcommands, you can see which ones by using the `-h` or `--help` flag at any level.

For example, if we wanted to create a schema, a command could look something like:

```
siera --copy schema create --name "College Degree" --attribute Grade
# Output: schema id
```

Where global option `--copy` copies the output of a command that results in output to your clipboard.

But options `--name` and `--attribute` are specific to the subcommand `schema create`.

### Know what to do (--help)

Every subcommand has the option of a `-h` or `--help` flag, where you'll see the usage of that subcommand, a description of the options as well as which options are required. We recommend playing around with these and trying commands out as the easiest way to get aquainted with the CLI. &#x20;

### Know what is happening (--verbose)

We believe that good logging makes for a good CLI user experience, but we also believe a productivity focused tool should not flood the terminal with logging. That is why Siera uses different log-levels.

| CLI flags                                 | Rust logger | Description                           |
| ----------------------------------------- | ----------- | ------------------------------------- |
| `--verbose` or `-v`                       | `info!`     | up to **informational**-level logging |
| `--verbose --verbose` or `-vv`            | `debug!`    | up to **debug**-level logging         |
| `--verbose --verbose --verbose` or `-vvv` | `trace!`    | up to **trace**-level logging         |

This means **by default** we will only log command output, warnings and errors. If you're looking for more info, use the more extensive verbosity levels.

---
