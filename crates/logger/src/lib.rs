//! Logger for this crate
//! Only cli bindings for now

#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_docs_in_private_items
)]

use clipboard::{ClipboardContext, ClipboardProvider};
use colored::Colorize;
use serde::Serialize;
use std::fmt;
use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

/// Macro module
#[macro_use]
pub mod macros;

/// Loglevel in the cli
#[derive(PartialEq, Eq, PartialOrd)]
pub enum LogLevel {
    /// Do not log any additional data
    Off,

    /// Only log errors
    Error,

    /// Log warnings and above
    Warn,

    /// Log info and above
    Info,

    /// Log debug and above
    Debug,

    /// Log trace and above
    Trace,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Error => "error".bold().red(),
            Self::Warn => "warn".bold().yellow(),
            Self::Info => "info".bold().cyan(),
            Self::Debug => "debug".bold().blue(),
            Self::Trace => "trace".bold().purple(),
            Self::Off => "off".green(),
        };
        write!(f, "{}", s)
    }
}

/// Simple state of the logger
pub struct LoggerState {
    /// Whether the logger is already initialized
    pub init: bool,

    /// Whether the output that is being logged should also be copied
    pub should_copy: bool,

    /// The loglevel at the cli
    pub level: LogLevel,
}

impl LoggerState {
    /// Initialize the logger state
    #[must_use]
    pub const fn new(init: bool, should_copy: bool, log_level: LogLevel) -> Self {
        Self {
            init,
            should_copy,
            level: log_level,
        }
    }
}

lazy_static! {
/// Initialization of the state with default
    pub static ref STATE: RwLock<LoggerState> = RwLock::new(LoggerState::new(false, false, LogLevel::Off));
}

/// Initialize the logger
///
/// # Panics
///
/// When the logger is already initialized
pub fn init(level: LogLevel, should_copy: bool) {
    assert!(
        !STATE.read().unwrap().init,
        "Logger should only be initialized once!"
    );

    let mut state = STATE.write().unwrap();
    state.init = true;
    state.level = level;
    state.should_copy = should_copy;
}

/// Prettify any string that implements Serialize
///
/// # Panics
///
/// When the object could not be serialized
pub fn pretty_stringify_obj(obj: impl Serialize) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

    obj.serialize(&mut ser).unwrap();

    String::from_utf8(ser.into_inner()).unwrap()
}

/// Copy any output to the clipboard in an OS agnostic way
///
/// # Panics
///
/// When the clipoard provider could not be found
pub fn copy_to_clipboard(string: impl AsRef<str>) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(string.as_ref().to_owned()).unwrap();
}
