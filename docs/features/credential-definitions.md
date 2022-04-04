---
description: Retrieve or create credential definitions
---

# Credential Definition

Aries CLI offers the following methods for credential definitions:

### Usage

```
aries-cli credential-definition [OPTIONS] [SUBCOMMAND]
```

#### Options

| Alias | Flag   | Description            |
| ----- | ------ | ---------------------- |
| -h    | --help | Print help information |

#### Subcommands

| Command | Description                          |
| ------- | ------------------------------------ |
| create  | Create a new credential definition   |
| list    | List all your credential definitions |

### Create&#x20;

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
aries-cli -c credential-definition create -s WVqppUv9X3WyWGrbns5Uia:2:Example:1.0
```

### List

List all your current connections

```
aries-cli connection list [OPTIONS]
```

#### Available flags

| Alias | Flag       | Description            |
| ----- | ---------- | ---------------------- |
| -h    | --help     | Print help information |
| -i    | --id \<ID> | Get a connection by id |

#### Example usage

Supply a connection ID to get the connection record.

```
aries-cli connection list --id 851e2d1b-acee-4a71-b798-8d02a8addd09
```

###
