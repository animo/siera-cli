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
<a href="#getting-started">Getting started</a> &nbsp;|&nbsp;
  <a href="#common-examples">Common examples</a> &nbsp;|&nbsp;
  <a href="#roadmap">Roadmap</a> &nbsp;|&nbsp;
  <a href="#contributing">Contributing</a> 
    
</p>

<!-- Add badges? -->

Aries CLI is the most convenient way for self-sovereign identity (SSI) developers to interact with SSI agents.

* 🌐 **Environments** to easily manage configuration for multiple projects and agents
* 🌟 **Actions and workflows** that you can perform against an agent
* 💅🏻 **Mock data** so that you can focus on the important task of building your application instead of other foobar (coming soon 🚧)

If you are looking for more information about the concepts, example code and tutorials on how to use the CLI we recommend you check out our extensive [docs](https://github.com/animo/aries-cli/pull/www.google.com).


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

### Binaries

See [binaries](https://github.com/animo/aries-cli/releases).

### Setting up your environment

```sh
aries-cli configuration initialize
# > /location/of/the/config/file
```

This command will set up the community agent. To set up your own agent edit
the configuration file by adding your agent URL.

If you are getting started with the tool we recommend enabling informational logs by
passing the `--verbose` (or `-v`).

## Common examples

To see all actions simply use the `--help` or `-h` flag. Here are some common actions.

### Create a credential offer

```sh
aries-cli execute offer-credential
```

 Get a credential offer in your wallet &mdash; this command will execute all of the actions needed.


### Create an invitation for the toolbox

```sh
aries-cli --copy --verbose connections invite --toolbox
```

The `--toolbox` flag creates an invitation for the [Toolbox](https://github.com/hyperledger/aries-toolbox).

The `--copy` flag will copy the invite URL to your clipboard so it can easily be pasted in the toolbox

Optionally, add the `--quiet` flag to suppress output to stdout.

For more options under the `connections invite` subcommand see:

```
aries-cli connections invite --help
```


## Roadmap

<!-- TODO: Add more details about the actions and features we support -->
We intend to support multiple versions of the Aries agent. See the CLI help `aries-cli --help` for a list of actions we currently support.

Next we are looking at adding:

| Feature          | Status | Description                                                                           |
| ---------------- | ------ | ------------------------------------------------------------------------------------- |
| Mock data        | 🚧      | Generate mock data for large data structures like schemas and credential definitions. |
| Filters          | 🚧      | Use filters to determine what server output you want returned.                        |
| execute offer-credential  |   ✅    | Execute sequence of actions to get a credential offer in your wallet.      |
| Present proof    | 🚧      | Present proofs.                                                                       |

## Contributing

Is there something you'd like to fix or add to the CLI? Great! We 💗 community
contributions. [Get involved](./docs/contributing.md).


