---
description: Retrieve or create schemas
---

# Schema's

Aries CLI offers the following methods for schema's:

### Usage

```
aries-cli schemas [OPTIONS] [SUBCOMMAND]
```

#### Options

| Alias | Flag       | Description                  |
| ----- | ---------- | ---------------------------- |
| -h    | --help     | Print help information       |
| -i    | --id \<ID> | ID of the schema to retrieve |

#### Subcommands

| Command | Description            |
| ------- | ---------------------- |
| help    | Print help information |
| create  | Create a new schema    |



### Create a schema

Create a new schema.

```
aries-cli schemas create [OPTIONS] --name <NAME> 
```

#### Available flags

| Alias | Flag                       | Description                                                                                                 |
| ----- | -------------------------- | ----------------------------------------------------------------------------------------------------------- |
| -h    | --help                     | Print help information                                                                                      |
| -n    | --name \<NAME>             | Name of the schema                                                                                          |
| -a    | --attributes \<ATTRIBUTES> | Keys that describe the structure of the schema - for example "age"                                          |
| -v    | --version \<VERSION>       | Version of of the schema, useful to be able to specify multiple versions of the same schema \[default: 1.0] |

#### Example usage

Create a new schema with the properties `name` and `age`. The `-c` flag automatically copies the ID of the created schema to your clipboard.

```
aries-cli -c schemas create -n example -a name -a age
```
