<p align="center">
  <br />
  <img
    alt="Aries CLI logo"
    src="https://raw.githubusercontent.com/hyperledger/aries-framework-javascript/aa31131825e3331dc93694bc58414d955dcb1129/images/aries-logo.png"
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

Building an SSI solution requires many, _many_ interactions with an SSI agent. Each interaction, often comprised of multiple steps, must be furnished with an endpoint and associated data. The Aries CLI provides for each of these needs by giving users:

* 🌐 **Environments** to easily manage configuration for multiple projects and agents
* 🌟 **Actions and workflows** that you can perform against an agent (see our [support matrix](./docs/support_matrix.md))
* 💅🏻 **Mock data** so that you can focus on the important task of building your application instead of other foobar (coming soon 🚧)

## Installation
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
community agent as your default environment.

## Common examples

### Creating an invitation for the toolbox

```sh
aries-cli --copy --quiet connections invite --toolbox
```

We wanted a quick-fire way of creating an invite to the toolbox so that we can
continue with the task at hand: testing an invitation workflow for our app. This
command takes care of all the plumbing for us:

The `--toolbox` flag makes sure the invite has an alias as Toolbox, sets auto accept to true and sets the toolbox as admin for the invite.

The `--copy` flag will copy the invite URL to your clipboard so it can easily be pasted in the toolbox

Optionally, add the `--quiet` flag to suppress output to stdout.

For more options under the `connections invite` subcommand see:

```
aries-cli connections invite --help
```

### Another descriptive name for example

insert details

## Contributing

Is there something you'd like to fix or add to the CLI? Great! We 💗 community
contributions. See [this doc](./docs/contributing.md) to get started.

## Support matrix

We intend to support every version of the Aries agent. See [this catalogue](./docs/support_matrix.md) of the features we currently support.
