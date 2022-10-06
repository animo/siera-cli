---
description: Siera allows you to add and choose environments through subcommands.
---

# Configuration

{% hint style="info" %}
See the guide on how to [set up Siera](../guides/configuration.md) for more information on how to add and change environments relating to your agent configuration.&#x20;
{% endhint %}

Siera supports using different environments. An environment consists of an environment name, an agent URL, an API key (optional) and a token (optional). A specific environment to use can be specified in a command using the `--environment <ENVIRONMENT>` option.&#x20;

```yaml
---
configurations:
  default:
    endpoint: "https://agent.community.animo.id"
    api_key: ~
    token: ~
```

{% hint style="warning" %}
In MS Windows Powershell you need to either run your command prompt application as administrator or allow your current user write access to `C:\Program Files\Common Files\` in order to be able to write a config file to disk. This does not apply when using WSL.&#x20;
{% endhint %}

### Usage

```
siera configuration [SUBCOMMAND]
```

#### Options

| Alias | Flag     | Description            |
| ----- | -------- | ---------------------- |
| `-h`  | `--help` | Print help information |

#### Subcommands

| Command | Description                                                            |
| ------- | ---------------------------------------------------------------------- |
| add     | Add a new, or overwrite an existing, agent to your configuration file. |
| view    | Print your current configuration file.                                 |

### Add

Add a new, or overwrite an existing, agent to your configuration file.

```
siera configuration add [OPTIONS]
```

#### Available flags

| Alias | Flag                          | Description                                                                  |
| ----- | ----------------------------- | ---------------------------------------------------------------------------- |
| `-a`  | --api-key \<API_KEY>          | This API key will be passed to the agent.                                    |
| `-d`  | --default                     | Add the default agent to the configuration (can be combined with `--token`). |
| `-e`  | `--environment <ENVIRONMENT>` | Specify your current environment.                                            |
| `-h`  | `--help`                      | Print help information.                                                      |
| `-t`  | `--token <TOKEN>`             | Authentication token for a multitenancy agent.                               |
| `-u`  | `--agent-url <AGENT_URL>`     | The Agent agent URL that requests will be sent to.                           |

#### Example usage

Add a custom environment to your configuration file.&#x20;

```
siera configuration add --environment=<YOUR_ENV_NAME> --agent-url=<YOUR_AGENT_URL> --api-key=<YOUR_API_KEY> --token=<YOUR_TOKEN>
```

Siera uses the 'default' environment when no `--environment` flag is given with commands. If you want to use another environment as your default, you can override your current default environment by specifying `--environment=default` in the above command.

### View

Print your current configuration file and its path.

```
siera configuration view
```

#### Available flags

| Alias | Flag     | Description             |
| ----- | -------- | ----------------------- |
| `-h`  | `--help` | Print help information. |

#### Example usage

The command results in a print out of both the configuration path and the `config.yaml` itself.&#x20;
