---
description: >-
  There are several ways to install the Aries CLI, depending on your system and
  preferences. Currently the methods are limited, but we're working on adding
  more.
---

# Installation

### Installing through package managers

Currently, the Aries CLI supports Brew as a package manager.

```
brew install aries-cli
```

For other systems than macOS, we recommend installing with Cargo, or installing through the binaries.

### Installing with Cargo

To install with Cargo, Rust needs to be installed on your system first. This method is suitable for every system, but might have some undocumented dependency errors depending on your system setup.

```
cargo install --git https://github.com/animo/aries-cli
```

This method of install will **not** update automatically and is therefore not recommended if another option is available to you. To update manually, reuse the installation command.

### Installing through binaries

The binaries can be downloaded from the[ website](https://aries-cli.animo.id) under the `Get Started` dropdown or from the [GitHub releases](https://github.com/animo/aries-cli/releases). This method of install will **not** update automatically and is therefore not recommended if another option is available to you. We have binaries available for:

* Windows (aries-cli.exe)
* macOS (Silicon) (aries-cli-macos-arm)
* macOS (Intel) (aries-cli-macos-x86\_64)
* Linux (aries-cli-linux-x86\_64)
