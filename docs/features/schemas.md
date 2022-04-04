---
description: Retrieve or create schemas
---

# Schema

Aries CLI offers the following methods for schema's:

### Usage

```
aries-cli schema [OPTIONS] [SUBCOMMAND]
```

#### Options

| Alias | Flag   | Description            |
| ----- | ------ | ---------------------- |
| -h    | --help | Print help information |

#### Subcommands

| Command | Description                   |
| ------- | ----------------------------- |
| create  | Create a new schema           |
| list    | List all your current schemas |

### Create a schema

Create a new schema.

```
aries-cli schema create [OPTIONS] --name <NAME> --attribute <ATTRIBUTE>
```

#### Available flags

| Alias | Flag                     | Description                                                                                                           |
| ----- | ------------------------ | --------------------------------------------------------------------------------------------------------------------- |
| -h    | --help                   | Print help information                                                                                                |
| -n    | --name \<NAME>           | Name of the schema                                                                                                    |
| -a    | --attribute \<ATTRIBUTE> | Keys that describe the structure of the schema - for example "age". Given in t following format: -a foo -a bar -a baz |
| -v    | --version \<VERSION>     | Version of of the schema, useful to be able to specify multiple versions of the same schema \[default: 1.0]           |

#### Example usage

Create a new schema with the properties `name` and `age`. The `-c` flag automatically copies the ID of the created schema to your clipboard.

```
aries-cli -c schema create -n example -a name -a age
```
