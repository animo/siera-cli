---
description: Retrieve or create credential definitions
---

# Credential Definitions

Aries CLI offers the following methods for credential definitions:

### Usage

```
aries-cli credential-definitions [OPTIONS] [SUBCOMMAND]
```

#### Options

| Alias | Flag       | Description            |
| ----- | ---------- | ---------------------- |
| -h    | --help     | Print help information |
| -i    | --id \<ID> | Get a connection by id |

#### Subcommands

| Command | Description                        |
| ------- | ---------------------------------- |
| help    | Print help information             |
| create  | Create a new credential definition |



### Create a credential definition

Create a new credential definition.

```
aries-cli credential-definition create --schema-id <SCHEMA_ID>
```

#### Available flags

| Alias | Flag        | Description                        |
| ----- | ----------- | ---------------------------------- |
| -h    | --help      | Print help information             |
| -s    | --schema-id | Schema ID to use in the definition |

#### Example usage

Create a credential definition with a schema ID. The `-c` flag automatically copies the created credential definition to your clipboard.

```
aries-cli -c credential-definitions create -s WVqppUv9X3WyWGrbns5Uia:2:Example:1.0
```
