<p align="center">
  <br />
  <img
    alt="Aries CLI logo"
    src="./images/aries-cli-dark.svg#gh-dark-mode-only"
    height="250px"
  />
   <img
    alt="Aries CLI logo"
    src="./images/aries-cli-light.svg#gh-light-mode-only"
    height="250px"
  />
</p>


<h1 align="center" ><b>Aries CLI</b></h1>


<h3 align="center">Powered By &nbsp; <img src="./images/animo-logo-dark-background.png#gh-dark-mode-only" height="12px"><img src="./images/animo-logo-light-background.png#gh-light-mode-only" height="12px"></h3><br>



<p align="center">
<a href="#roadmap">Roadmap</a> &nbsp;|&nbsp;
  <a href="#getting-started">Getting started</a> &nbsp;|&nbsp;
  <a href="#common-examples">Common examples</a> &nbsp;|&nbsp;
   <a href="#contributing">Contributing</a> 
    
</p>

<!-- Add badges? -->

Aries CLI is the most convenient way for self-sovereign identity (SSI) developers to interact with SSI agents.

Building an SSI solution requires many, _many_ interactions with an SSI agent. Each interaction, often comprised of multiple steps, must be furnished with an endpoint and associated data. The Aries CLI makes working with verifiable credentials easy by giving users:

* 🌐 **Environments** to easily manage configuration for multiple projects and agents
* 🌟 **Actions and workflows** that you can perform against an agent
* 💅🏻 **Mock data** so that you can focus on the important task of building your application instead of other foobar (coming soon 🚧)

This README was set up for you to get started as fast as possible. If you are looking for more information about the concepts, example code and tutorials on how to use the CLI we recommend you check out our extensive [docs](https://github.com/animo/aries-cli/pull/www.google.com).

## Roadmap

<!-- TODO: Add more details about the actions and features we support -->
We intend to support multiple versions of the Aries agent. See the CLI help `aries-cli --help` for a list of actions we currently support.

Next we are looking at adding:

| Feature          | Status | Description                                                                           |
| ---------------- | ------ | ------------------------------------------------------------------------------------- |
| Mock data        | 🚧      | Generate mock data for large data structures like schemas and credential definitions. |
| Filters          | 🚧      | Use filters to determine what server output you want returned.                        |
| Workflows        | 🚧      | Chain multiple actions together for higher-level goals like: issue a credential.      |
| Present proof v2 | 🚧      | Present proofs.                                                                       |


## Getting started

Heres how to install Aries-CLI using the most popular package managers. For advanced installation options, binaries and troubleshooting we recommend checking out the [installation docs](./docs/advanced_installation.md).

### macOS using Brew

```sh
echo "Coming soon!"
```

### Linux using Apt-get

```sh
echo "Coming soon!"
```

### Windows using Chocolatey

```powershell
Write-Output "Coming soon!"
```

### Setting up your environment

```sh
aries-cli configuration initialize
```

We highly recommend using `environment`s to avoid the repetitive task of
specifying agent URLs. With this CLI you can get up and running with our
community agent as your default environment, or use your own agent.

If you are getting started with the tool, we recommend enabling informational logs by
passing the `--verbose` (or `-v`) flag to get more information about what the CLI is
doing. We also support stacking verbosity up to three levels for debug logs: `-vv` and `-vvv`.

## Common examples

Below you will find some of the most common useful commands within the Aries CLI. To see all options, simply use the `--help` or `-h` flag.

### Creating an invitation for the toolbox

```sh
aries-cli --copy --verbose connections invite --toolbox
```

With this quick-fire way of creating an invite to the [Aries toolbox](https://github.com/hyperledger/aries-toolbox) you can continue
with the task at hand: testing an invitation workflow for your app. This
command takes care of all the plumbing.

The `--toolbox` flag makes sure the invite has an alias as Toolbox, sets auto accept to true and sets the toolbox as admin for the invite.

The `--copy` flag will copy the invite URL to your clipboard so it can easily be pasted in the toolbox

Optionally, add the `--quiet` flag to suppress output to stdout.

For more options under the `connections invite` subcommand see:

```
aries-cli connections invite --help
```

## Contributing

Is there something you'd like to fix or add to the CLI? Great! We 💗 community
contributions. [Get involved](./docs/contributing.md).


