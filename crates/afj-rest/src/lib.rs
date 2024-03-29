//! AFJ REST controller. This is a simple wrapper around the endpoints of an AFJ REST agent. It is
//! a minimal wrapper and does not do a lot of conversion and just sends it back to any frontend
//! which relies on this.

#[macro_use]
extern crate siera_logger;

#[macro_use]
/// Macros used within this crate
pub mod macros;

/// All of the submodule functionality of an AFJ REST agent
pub mod cloudagent;

/// An AFJ REST structure
pub mod agent;

/// Simple web bindings for the module to interact with the cloudagent
mod web;
