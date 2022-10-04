//! Aries cloudagent Python controller. This is a simple wrapper around the
//! endpoints of an Aries cloudagent Python. It is a minimal wrapper and does
//! not do a lot of conversion and just sends it back to any frontend which
//! relies on this.

#![deny(clippy::missing_docs_in_private_items)]

#[macro_use]
extern crate siera_logger;

/// All of the submodule functionality of an Aries cloudagent Python
pub mod cloudagent;

/// Simple web bindings for the module to interact with the cloudagent
mod web;

/// TODO
pub mod agent;

/// Macro module
#[macro_use]
mod macros;
