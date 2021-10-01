# Aries Data Generator Toolkit

## Goal

The goal of this toolkit is to quickly interact with an Aries agent, ACA-Py only, for data manipulation. It MUST be possible to create a connection,
issue a credential, present a proof, batch every call, via a configuration
file.

## Design

The toolkit should be written in a 100% safe, extensible and verbose way. It
should also use as much default values as possible. This will help prevent
setting annoying irrelevant variables.

### Config

Values that are not defined by `src/typing.rs` MUST be ignored.

### Agent

You could simply supply an endpoint that points to an restul endpoint of an
agent, ACA-Py only for now. All the other functionality will happen simply
through this endpoint.

### Connections

For connections you could either supply a connection id or an invitiation qr +
url will be created.

If a connectionid is supplied it will continue to the next step.

If no connectionid is supplied it will log a qrcode and invitiation url and the
program will wait until the invitation has been accepted.

For now only one connection can be used at the same time. However the toolkit
should be written in an extensible way.

### Actions

Actions are the functions that are called on the `connection_id`. this could
be, but not limited to, issuing a credential, requesting a proof, basic
messaging, etc.
