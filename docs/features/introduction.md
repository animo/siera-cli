---
description: An introduction to all available options and subcommands.
---

# Overview

Siera was created to assist during practical day-to-day development tasks. Its features and subcommands reflect this.

### Usage

```
siera [OPTIONS] <SUBCOMMAND>
```

### Options

Siera offers the following options.

| Alias | Flag                          | Description                                                                 |
| ----- | ----------------------------- | --------------------------------------------------------------------------- |
| `-a`  | `--api-key <API_KEY>`         | This API key will be passed to the agent                                    |
| `-c`  | `--copy`                      | Copy output to your clipboard                                               |
| `-e`  | `--environment <ENVIRONMENT>` | Specify your current environment \[default: default]                        |
| `-h`  | `--help`                      | Print help information                                                      |
| `-o`  | `--config <CONFIG>`           | Supply a path to your configuration file to use that instead of the default |
| `-q`  | `--quiet`                     | Suppresses most output                                                      |
| -`t`  | `--token <TOKEN>`             | Authentication token for a multi tenancy agent                              |
| -`u`  | `--agent-url <AGENT_URL>`     | The Agent agent URL that requests will be sent to                           |
| `-v`  | `--verbose`                   | Print debug logs                                                            |
| `-V`  | `--version`                   | Print version information                                                   |

### Subcommands

Siera offers the following subcommands.

| Subcommand                                         | Description                                                                                                                                                    |
| -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [automate](../automations/introduction.md)         | Automated actions that combine multiple functions                                                                                                              |
| [configuration](environments.md)                   | Add agents to your configuration or view your current configuration. To quickly get started, run the following command: `siera configuraton add --default` |
| [connection](connections.md)                       | Retrieve connections or create invitations                                                                                                                     |
| [credential](credentials.md)                       | Offer or propose credentials                                                                                                                                   |
| [credential-definition](credential-definitions.md) | Retrieve or create credential definitions                                                                                                                      |
| feature                                            | List all available features                                                                                                                                    |
| [message](messages.md)                             | Send a secure message to an exist connection                                                                                                                   |
| [schema](schemas.md)                               | Retrieve or create schemas                                                                                                                                     |
