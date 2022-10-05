---
description: Retrieve connections or create invitations
---

# Connection

Siera offers various methods to create and receive invitations.

### Usage

```
siera connection [OPTIONS] [SUBCOMMAND]
```

#### Options

| Alias | Flag     | Description            |
| ----- | -------- | ---------------------- |
| `-h`  | `--help` | Print help information |

#### Subcommands

| Command | Description                        |
| ------- | ---------------------------------- |
| invite  | Create a new connection invitation |
| list    | List all your current connections  |
| receive | Receive an invitation by url       |

### Invite

Create a new connection invitation

```
siera connection invite [OPTIONS]
```

#### Available flags

| Alias | Flag              | Description                                                                                                                                                                                                                                                             |
| ----- | ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-a`  | `--auto-accept`   | Automatically accept the new connection once they accept this invitation                                                                                                                                                                                                |
| `-h`  | `--help`          | Print help information                                                                                                                                                                                                                                                  |
| `-l`  | `--alias <ALIAS>` | The name a new connection will use to identify itself                                                                                                                                                                                                                   |
| `-m`  | `--multi-use`     | This invitation can be used more than once                                                                                                                                                                                                                              |
| `-q`  | `--qr`            | Print a QR code, convenient for use with mobile apps                                                                                                                                                                                                                    |
| `-t`  | `--toolbox`       | <p>Short-hand to create an invitation for the Aries Toolbox that sets:</p><ul><li><code>alias="toolbox"</code></li><li><code>multi-use="false"</code></li><li><code>auto-accept="true"</code></li></ul><p>and gives admin rights over the invitation to the toolbox</p> |

#### Example usage

Create an invitation that can be used more than once and is auto accepted. The `-c` flag automatically copies the invitation URL to your clipboard.

```
siera -c connection invite -m -a
```

### List

List all your current connections

```
siera connection list [OPTIONS]
```

#### Available flags

| Alias | Flag        | Description            |
| ----- | ----------- | ---------------------- |
| `-h`  | `--help`    | Print help information |
| `-i`  | `--id <ID>` | Get a connection by id |

#### Example usage

Supply a connection ID to get the connection record.

```
siera connection list --id 851e2d1b-acee-4a71-b798-8d02a8addd09
```

### Receive

Receive an invitation by URL.

```
siera connection receive --url <URL>
```

#### Available flags

| Alias | Flag          | Description                                                    |
| ----- | ------------- | -------------------------------------------------------------- |
| `-h`  | `--help`      | Print help information                                         |
| `-u`  | `--url <URL>` | Receive an invitation with the invitation url, between quotes. |

#### Example usage

Supply an invitation URL to accept. Make sure the URL is between quotes.

```
siera connection receive --url "https://didcomm.agent.community.animo.id?c_i=eyJAdHlwZSI6ICJkaWQ6c292OkJ6Q2JzTlloTXJqSGlxWkRUVUFTSGc7c3BlYy9jb25uZWN0aW9ucy8xLjAvaW52aXRhdGlvbiIsICJAaWQiOiAiMjNiOGY0ZDAtNzIyNi00ZmQ0LWEyNDAtMjJkNDgxNTViODBlIiwgInJlY2lwaWVudEtleXMiOiBbIjZZVVU2dnp2b0hTV29OWlRDUGE1eFlYV3kyUGJ5VGREcnVKa0VMRXR4NW9kIl0sICJsYWJlbCI6ICJBbmltbyBDb21tdW5pdHkgQWdlbnQiLCAic2VydmljZUVuZHBvaW50IjogImh0dHBzOi8vZGlkY29tbS5hZ2VudC5jb21tdW5pdHkuYW5pbW8uaWQifQ=="
```
