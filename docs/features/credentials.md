---
description: Offer or propose credentials
---

# Credential

Aries CLI offers the following methods for credentials:

### Usage

```
aries-cli credential [SUBCOMMAND]
```

#### Options

| Alias | Flag   | Description            |
| ----- | ------ | ---------------------- |
| -h    | --help | Print help information |

#### Subcommands

| Command | Description                                                             |
| ------- | ----------------------------------------------------------------------- |
| offer   | Offer a new credential to an existing connection                        |
| propose | Not implemented yet: propose a credential that should be offered to you |

### Offer

Offer a new credential to an existing connection

```
aries-cli credential offer [OPTIONS] --connection-id <CONNECTION_ID> --cred-def-id <CRED_DEF_ID>
```

#### Available flags

| Alias | Flag                              | Description                                       |
| ----- | --------------------------------- | ------------------------------------------------- |
| -h    | --help                            | Print help information                            |
| -c    | --cred-def-id \<CRED\_DEF\_ID>    | A credential definition to base the credential on |
| -i    | --connection-id \<CONNECTION\_ID> | Existing connection ID to offer the credential to |
| -k    | --key \<KEY>                      | An attribute key name                             |
| -v    | --value \<VALUE>                  | An attribute value                                |

#### Example usage

Offer a credential to an existing connection. The following command offers a credential with key `name` and value `animo`.

```
aries-cli credentials offer -i d583caa1-0bdd-46f9-98d3-8d0bdd4a6056 -c Ehx3RZSV38pn3MYvxtHhbQ:3:CL:213800:default -k name -v animo
```
