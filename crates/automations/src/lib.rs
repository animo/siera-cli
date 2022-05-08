//! Crate that contains the automations that any cloudagent can execute. This
//! includes scripts like automatically create a connection, register the
//! schema and credential definition and issue the credential afterwards.

#![deny(clippy::missing_docs_in_private_items)]

/// Automation module that contains every automation script
pub mod automations;

/// Error module that includes the user-level errors and the result type
mod error;
