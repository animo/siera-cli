---
description: Send a secure message to an existing connection
---

# Message

Siera offers the following methods for messages:

### Usage

```
siera message --connection-id <ID> --message <MESSAGE>
```

#### Options

| Alias | Flag                   | Description                          |
| ----- | ---------------------- | ------------------------------------ |
| `-h`  | `--help`               | Print help information               |
| `-i`  | `--connection-id <ID>` | Connection ID to send the message to |
| `-m`  | `--message <MESSAGE>`  | Contents of the message              |

#### Example usage

The following commands sends `hello` to an existing connection:

```
siera message -i d583caa1-0bdd-46f9-98d3-8d0bdd4a6056 -m hello
```
