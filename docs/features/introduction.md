---
description: An overview of all available options and subcommands.
---

# Introduction

### Usage

```
aries-cli [OPTIONS] <SUBCOMMAND>
```

### Options

| Alias | Flag                         | Description                                                                 |
| ----- | ---------------------------- | --------------------------------------------------------------------------- |
| -a    | --api-key \<API\_KEY>        | This API key will be passed to the agent                                    |
| -c    | --copy                       | Copy output to your clipboard                                               |
| -e    | --environment \<ENVIRONMENT> | Specify your current environment \[default: default]                        |
| -h    | --help                       | Print help information                                                      |
| -o    | --config \<CONFIG>           | Supply a path to your configuration file to use that instead of the default |
| -q    | -quiet                       | Suppresses most output                                                      |
| -t    | --token \<TOKEN>             | Authentication token for a multi tenancy agent                              |
| -u    | --agent-url \<AGENT\_URL>    | The Aries agent URL that requests will be sent to                           |
| -v    | --verbose                    | Print debug logs                                                            |
| -V    | --version                    | Print version information                                                   |

### Subcommands

Aries CLI offers the following subcommands:

| Subcommand            | Description                                                                                                                                                  |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| automate              | Automated actions that combine multiple functions                                                                                                            |
| configuration         | Add agents to your configuration or view your current configuration. To quickly get started, run the following command: aries-cli configuraton add --default |
| connection            | Retrieve connections or create invitations                                                                                                                   |
| credential            | Offer or propose credentials                                                                                                                                 |
| credential-definition | Retrieve or create credential definitions                                                                                                                    |
| feature               | List all available features                                                                                                                                  |
| message               | Send a secure message to an exist connection                                                                                                                 |
| schema                | Retrieve or create schemas                                                                                                                                   |
