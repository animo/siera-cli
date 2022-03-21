<p align="center">
  <br />
  <img
    alt="Aries CLI logo"
    src="https://github.com/animo/aries-cli/blob/ea9589cafa34a1375a7847995fcec2b000141b97/images/aries-cli.png"
    height="250px"
  />
</p>
<h1 align="center"><b>Aries CLI</b></h1>
<p align="center">
  <a href="#getting-started">Getting started</a> &nbsp;|&nbsp;
  <a href="#common-examples">Common examples</a> &nbsp;|&nbsp;
   <a href="#contributing">Contributing</a> &nbsp;|&nbsp;
    <a href="#roadmap">Roadmap</a> &nbsp;|&nbsp;
</p>


> Powered by [Animo](https://animo.id) solutions

<!-- Add badges? -->

Aries CLI is the most convenient way for self-sovereign identity (SSI) developers to interact with SSI agents.

Building an SSI solution requires many, _many_ interactions with an SSI agent. Each interaction, often comprised of multiple steps, must be furnished with an endpoint and associated data. The Aries CLI makes working with verifiable credentials easy by giving users:

* 🌐 **Environments** to easily manage configuration for multiple projects and agents
* 🌟 **Actions and workflows** that you can perform against an agent
* 💅🏻 **Mock data** so that you can focus on the important task of building your application instead of other foobar (coming soon 🚧)

This README was set up for you to get started as fast as possible. Looking for more information about the concepts, example code and tutorials on how to use the CLI? Check out our extensive [docs](https://github.com/animo/aries-cli/pull/www.google.com).

## Getting started

Whatever your system, we've got you covered.
### macOS
#### Binary
[Download the binary](https://github.com/animo/aries-cli/releases) and place it on your PATH.

#### Brew
```sh
echo "Coming soon!"
```

### Linux
#### Binary
[Download the binary](https://github.com/animo/aries-cli/releases) and place it on your PATH.

#### Apt-get
```sh
echo "Coming soon!"
```

### Windows

#### Binary
[Download the binary](https://github.com/animo/aries-cli/releases) and place it on your PATH

#### Chocolatey

```powershell
Write-Output "Coming soon!"
```

### Advanced installation

See [this document](./docs/advanced_installation.md) for advanced installation options.

## Setting up your environment

```sh
aries-cli configuration initialize
```

We highly recommend using `environment`s to avoid the repetitive task of
specifying agent URLs. With this CLI you can get up and running with our
community agent as your default environment, or use your own agent.

## Common examples

Below you will find some of the most common useful commands within the Aries CLI. To see all options, simply use the `--help` or `-h` flag.

### Creating an invitation for the toolbox

```sh
aries-cli --copy --quiet connections invite --toolbox
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

## Roadmap

<!-- TODO: Add more details about the actions and features we support -->
We intend to support multiple versions of the Aries agent. See the CLI help `aries-cli --help` for a list of actions we currently support.

Next we are looking at adding:

| Feature | Status | Description |
|---------|--------|-------------|
| Mock data | 🚧 | Generate mock data for large data structures like schemas and credential definitions |
| Workflows | 🚧 | Chain multiple actions together for higher-level goals like: issue a credential |
