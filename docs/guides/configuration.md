---
description: >-
  The Aries CLI is basically good to go with a single initialization command,
  however there are some nice additional configuration options like connecting
  your own agent or using a tenant agent.
---

# Configuration

### Basic configuration

After installation, the CLI prompts you to initialize your configuration with the following command.

```
aries-cli configuration add --default
```

This command creates the following `config.yaml` configuration file. It is important to note that you don't have to do anything with this file. You can get started exploring the features right away with the CLI as it is.

```yaml
configurations: 
    default: 
        endpoint: "https://agent.community.animo.id" 
        api_key: ~ 
        token: ~ 
```

The config file contains an endpoint for the agent that handles all of your CLI actions. By default, this is the URL for the Animo community agent. Using this agent, you can get started right away without any further setup.

You might find, however, that you want some more advanced configuration in order to avoid the clutter of the community agent (as you will encounter the actions of everyone using it). No worries! In a few easy steps, you can set up your own tenant environment or connect your own agent instead.

### Advanced configuration - use our multitenant agent

With a tenant you don't have to worry about the connections, schema's and credentials of other community members. You get your own agent to use however you like.

To use our multitenant agent to set up your own tenant, simply:

* Choose to [claim your token via the Aries CLI website](https://aries-cli.animo.id).
* Connect your GitHub account by following the instructions that pop up.
* Copy the command that appears and run it in your terminal.

If you've initialized the CLI before, the 'default' environment in your `config.yaml` file will now be overwritten with a new default environment. If you haven't initialized the CLI before, the `config.yaml` will be created with the multitenant default environment.

```yaml
configurations: 
    default: 
        endpoint: "https://agent.ssi.community" 
        api_key: ~ 
        token: <YOUR_TOKEN>
```

This new environment contains the endpoint for the agent that handles all of your CLI actions (in this case, the URL of our tenant agent). It also contains the custom token you claimed by connecting your GitHub account. You can now use a subtenant of the Animo community agent to execute all CLI commands.

The Aries CLI uses the 'default' environment when no `--environment <ENVIRONMENT>` flag is given. Having your own tenant will help a lot with keeping your development process clear. However, if you already have a development agent, you might want to consider either adding an environment for it or switching over to it completely.

### Additional configuration - use your own agent

The Aries CLI supports using different [environments](../features/environments.md). The initialization command creates the following `config.yaml` configuration file, containing only the default environment that uses the community agent.

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

