---
description: Offer or propose credentials
---

# Credential

Siera offers the following methods for credentials:

### Usage

```
siera credential [SUBCOMMAND]
```

#### Options

| Alias | Flag     | Description            |
| ----- | -------- | ---------------------- |
| `-h`  | `--help` | Print help information |

#### Subcommands

| Command | Description                                      |
| ------- | ------------------------------------------------ |
| offer   | Offer a new credential to an existing connection |

### Offer

Offer a new credential to an existing connection

```
siera credential offer [OPTIONS] --connection-id <CONNECTION_ID> --cred-def-id <CRED_DEF_ID>
```

#### Available flags

| Alias | Flag                              | Description                                       |
| ----- | --------------------------------- | ------------------------------------------------- |
| `-h`  | `--help`                          | Print help information                            |
| `-c`  | `--cred-def-id <CRED_DEF_ID>`     | A credential definition to base the credential on |
| `-i`  | `--connection-id <CONNECTION_ID>` | Existing connection ID to offer the credential to |
| `-k`  | `--key <KEY>`                     | An attribute key name                             |
| `-v`  | `--value <VALUE>`                 | An attribute value                                |

#### Example usage

Offer a credential to an existing connection. The following command offers a credential with key `name` and value `animo`.

```
siera credentials offer -i d583caa1-0bdd-46f9-98d3-8d0bdd4a6056 -c Ehx3RZSV38pn3MYvxtHhbQ:3:CL:213800:default -k name -v animo
```

The key and value flags are index matched and should be used as such. The below command matches key 1..3 to value 1..3.&#x20;

```
siera credential offer ... -k=key1 -v=value1 -k=key2 -k=key3 -v=value2 -v=value3
```
