---
description: Present proof with Siera.
---

# Proof

### Usage

```
siera proof <SUBCOMMAND>
```

#### Options

| Alias | Flag     | Description            |
| ----- | -------- | ---------------------- |
| `-h`  | `--help` | Print help information |

#### Subcommands

| Command | Description                       |
| ------- | --------------------------------- |
| request | Request a proof by connection id. |

### Request

Request a proof by connection id.

```
siera proof request [OPTIONS] --connection-id <CONNECTION_ID>
```

#### Available flags

| Alias | Flag                              | Description                                                                                     |
| ----- | --------------------------------- | ----------------------------------------------------------------------------------------------- |
| `-h`  | `--help`                          | Print help information                                                                          |
| `-a`  | `--attribute <ATTRIBUTE>`         | Attribute required in the proof request. eg. `-a=name -a=lastname`                              |
| `-c`  | `--connection-id <CONNECTION_ID>` | Connection id to send the proof request to.                                                     |
| `-n`  | `--name <NAME>`                   | Name of the proof request \[default: proof-request]                                             |
| `-p`  | `--predicate <PREDICATE>`         | Predicates required in the proof request (format = name,operator,value). e.g. `-p= "age,>=,18"` |

Pay extra attention to the usage of `--attribute` and `--predicate`.&#x20;

With the attribute flag, each attribute is flagged individually.

```
siera proof request -c <YOUR_CONNECTION_ID> -a attribute1 -a attribute2 -a attribute3
```

With the predicate flag, each predicate as a whole (the name, operator and value together) is flagged individually. The name, operator and value are comma separated and between quotes, resulting in `--predicate "name,operator,value"`.

```
siera proof request -c <YOUR_CONNECTION_ID> ... -p "date,>,20210101" -p "age,>,21"
```
