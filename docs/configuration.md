# Configuration file and environments

Providing an endpoint and api-key with every call can be rather annoying. So in order to create a clean api we have decided to add a configuration file to the tool. The default location of the configuration file on \*NIX systems is `~/.config/aries-cli/config.ini` and on Windows `TODO`.

## No Configuration file

Without a configuration file the `--endpoint` is required and the `--api-key` is only required if the cloudagent requires this.

**example**

```sh
aries-cli -e https://cloudagent.example.com --api-key 1234 invite --qr
```

## Own configuration file

A configuration file can be created and provided to the `--config` option. The configuration file is a `.ini` file and needs the following structure:

```ini
; Special environment. This will be used when no --environment is provided
[Default]
endpoint = https://cloudagent.example.com
api-key = 1234-5678 ; only needed if required by the cloudagent

; Choose a custom name that fits the cloudagent, e.g. work, staging, stable, etc.
[ENVIRONMENT]
endpoint = https://cloudagent-2.example.com

[ENVIRONMENT-2]
endpoint = https://cloudagent-3.example.com
```

**example**

```sh
aries-cli --config --environment ENVIRONMENT invite
```

## Default configuration file

<!-- TODO: NOT YET IMPLEMENTED -->

The default can be initialised with one of the two commands: `aries-cli init` and `aries-cli config --init`. These two commands create a default configuration file. On any \*NIX system it uses the following location: `~/.config/aries-cli/config.ini` and for Windows systems it uses `TODO` as the location.

The configuration is initialised with the following structure:

```ini
[Default]
endpoint = https://agent.community.animo.id
```

Because it resides at the default location, supplying the `--config` is not required anymore. This will allow for the simplest api:

```sh
aries-cli invite --qr
```

This is the same as the two following commands:

```sh
aries-cli --config=~/.config/aries-cli/config.ini invite --qr

aries-cli --endpoint=https://agent.community.animo.id invite --qr
```

The default configuration can be expanded with your own default cloud agent and other environments.

### Example configuration file

```ini
[Default]
endpoint = https://cloudagent.example.com

[Development]
endpoint = https://development.cloudagent.example.com

[Staging]
endpoint = https://staging.cloudagent.example.com
api-key = 1234-5678

[Release]
endpoint = https://release.cloudagent.example.com
api-key = 1234-5678
```

### Example commands

```sh
aries-cli invite -qr

aries-cli --environment Development schema --name foo --attribute bar

aries-cli --environment Staging connections --alias Toolbox
```
