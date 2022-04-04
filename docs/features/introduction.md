---
description: An overview of the available options and subcommands.
---

# Introduction

### Usage

```
aries-cli [OPTIONS] <SUBCOMMAND>
```

### Options

| Alias | Flag                         | Description                                          |
| ----- | ---------------------------- | ---------------------------------------------------- |
| -a    | --api-key \<API\_KEY>        | This API key will be passed to the agent             |
| -c    | --copy                       | Copy output to your clipboard                        |
| -e    | --environment \<ENVIRONMENT> | Specify your current environment \[default: default] |
| -h    | --help                       | Print help information                               |
| -o    | --config \<CONFIG>           | Path to your configuration file                      |
| -q    | -quiet                       | Suppresses most output                               |
| -t    | --token \<TOKEN>             | Authentication token for a multi tenancy agent       |
| -u    | --agent-url \<AGENT\_URL>    | The Aries agent URL that requests will be sent to    |
| -v    | --verbose                    | Print debug logs                                     |
| -V    | --version                    | Print version information                            |

### Subcommands

Aries CLI offers the following subcommands:

| Subcommand             | Description                                               |
| ---------------------- | --------------------------------------------------------- |
| configuration          | Initialize or view current configuration                  |
| connections            | Retrieve connections or create invitations                |
| credential-definitions | Retrieve or create credential definitions                 |
| credentials            | Offer or propose credentials                              |
| features               | List all available features                               |
| help                   | Print this message or the help of the given subcommand(s) |
| message                | Send a secure message to an exist connection              |
| schemas                | Retrieve or create schemas                                |
| workflow               | Automated actions that combine multiple functions         |
