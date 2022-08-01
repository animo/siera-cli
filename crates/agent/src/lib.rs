//! Crate that contains traits and structs to implement for an agent like aries-cloudagent-python
//! or aries-framework-javascript REST. This does not contain any functionality as that should be
//! handled by the other crates implementing this crate

#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_docs_in_private_items
)]
#![allow(clippy::struct_excessive_bools, clippy::module_name_repetitions)]

/// Error module that includes the user-level errors and the result type
pub mod error;

/// Traits and structures for a generic cloudagent
pub mod modules;
