---
description: >-
  There are several ways to install the Agent CLI, depending on your system and
  preferences. Currently  the methods are limited but we're working on adding
  more.
---

# Installation

### Installing through package managers

Currently, the Agent CLI supports Brew as a package manager.

```
brew tap animo/agent-cli
brew install agent-cli
```

For other systems than macOS we recommend installing with Cargo, or installing through the binaries.

### Installing with Cargo

To install with Cargo, Rust needs to be installed on your system first. This method is suitable for every system, but might have some undocumented dependency errors depending on your system setup.

```
cargo install --git https://github.com/animo/agent-cli
```

This method of install will **not** update automatically and is therefore not recommended if another option is available to you. To update manually, reuse the installation command.

### Installing through binaries

The binaries can be downloaded from the[ website](https://agent-cli.animo.id)`under`the `Get started` dropdown or from the [GitHub releases](https://github.com/animo/agent-cli/releases). This method of install will **not** update automatically and is therefore not recommended if another option is available to you. We have binaries available for:

- Windows (x86_64)
- macOS (x86_64 / arm)
- Linux (x86_64)
