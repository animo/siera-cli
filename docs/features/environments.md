---
description: The Aries CLI allows you to add and choose environments through subcommands.
---

# Configuration

{% hint style="info" %}
See the guide on how to [set up the Aries CLI](../guides/configuration.md) for more information on how to add and change environments relating to your agent configuration.&#x20;
{% endhint %}

The Aries CLI supports using different environments. The configuration can be&#x20;

```yaml
---
configurations:
  default:
    endpoint: "https://agent.community.animo.id"
    api_key: ~
    token: ~
```

You can add new environments by using the `configuration add` command and specifying the environment name, agent endpoint, API key (optional) and token (optional) in the `config.yaml`.

```
aries-cli configuration add --environment=<YOUR_ENV_NAME> --agent-url=<YOUR_AGENT_URL> --api-key=<YOUR_API_KEY> --token=<YOUR_TOKEN>
```

```yaml
---
configurations:
  default:
    endpoint: "https://agent.community.animo.id"
    api_key: ~
    token: ~
  <YOUR_ENV_NAME>:
    endpoint: <YOUR_AGENT_URL>
    api_key: <YOUR_API_KEY>
    token: <YOUR_TOKEN>
```

To use the new environment, simply use the `--environment <ENVIRONMENT>` flag with each of your commands.

The Aries CLI uses the 'default' environment when no `--environment` flag is given. If you want to use another environment as your default, you can override your current default environment by specifying `--environment= default` in the above command.





### Usage

```
aries-cli configuration [SUBCOMMAND]
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
aries-cli configuration add [OPTIONS] 
```

#### Available flags

| Alias | Flag                          | Description                                                                  |
| ----- | ----------------------------- | ---------------------------------------------------------------------------- |
| `-a`  | --api-key \<API\_KEY>         | This API key will be passed to the agent.                                    |
| `-d`  | --default                     | Add the default agent to the configuration (can be combined with `--token`). |
| `-e`  | `--environment <ENVIRONMENT>` | Specify your current environment.                                            |
| `-h`  | `--help`                      | Print help information.                                                      |
| `-t`  | `--token <TOKEN>`             | Authentication token for a multitenancy agent.                               |
| `-u`  | `--agent-url <AGENT_URL>`     | The Aries agent URL that requests will be sent to.                           |

### View

Print your current configuration file and its path.

```
aries-cli configuration view
```

#### Available flags

| Alias | Flag     | Description             |
| ----- | -------- | ----------------------- |
| `-h`  | `--help` | Print help information. |

