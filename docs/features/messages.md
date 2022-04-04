---
description: Send a secure message to an existing connection
---

# Message

Aries CLI offers the following methods for messages:

### Usage

```
aries-cli message --id <ID> --message <MESSAGE>
```

#### Options

| Alias | Flag                 | Description                         |
| ----- | -------------------- | ----------------------------------- |
| -h    | --help               | Print help information              |
| -i    | --id \<ID>           | Connection ID to sen the message to |
| -m    | --message \<MESSAGE> | Contents of the message             |

#### Example usage

The following commands sends `hello` to an existing connection:

```
aries-cli message -i d583caa1-0bdd-46f9-98d3-8d0bdd4a6056 -m hello
```
