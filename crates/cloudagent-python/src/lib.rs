//! Aries cloudagent Python controller. This is a simple wrapper around the
//! endpoints of an Aries cloudagent Python. It is a minimal wrapper and does
//! not do a lot of conversion and just sends it back to any frontend which
//! relays on this.

#![deny(clippy::missing_docs_in_private_items)]

/// All of the submodule functionality of an Aries cloudagent Python
pub mod agent_python;

/// An Aries Cloudagent Python structure
pub mod cloud_agent;

/// Simple web bindings for the module to interact with the cloudagent
mod web;

/// Macro module
#[macro_use]
mod macros;
