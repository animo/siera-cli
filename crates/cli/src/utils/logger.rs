use clipboard::{ClipboardContext, ClipboardProvider};
use log::LevelFilter;
use serde::Serialize;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

/// Simple state of the logger
#[derive(Default)]
pub struct LoggerState {
    /// Whether the logger is already initialized
    pub init: bool,

    /// Whether the output that is being logged should also be copied
    pub should_copy: bool,
}

/// Initialization of the state witg default
pub static mut STATE: LoggerState = LoggerState {
    init: false,
    should_copy: false,
};

/// Initialize the logger
pub fn init(level: LevelFilter, should_copy: bool) {
    unsafe {
        if STATE.init {
            panic!("Logger should only be initialized once!");
        }
        STATE.should_copy = should_copy;
    }
    let _ = TermLogger::init(
        level,
        ConfigBuilder::new()
            .set_location_level(LevelFilter::Debug)
            .set_thread_level(LevelFilter::Off)
            .set_time_level(LevelFilter::Debug)
            .set_target_level(LevelFilter::Debug)
            .set_max_level(LevelFilter::Debug)
            .add_filter_allow(String::from("agent_cli"))
            .add_filter_allow(String::from("cloudagent_"))
            .add_filter_allow(String::from("automation"))
            .build(),
        TerminalMode::default(),
        ColorChoice::Never,
    );
    unsafe {
        STATE.init = true;
    }
}

/// Prettify any string that implements Serialize
pub fn pretty_stringify_obj(obj: impl Serialize) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

    obj.serialize(&mut ser).unwrap();

    String::from_utf8(ser.into_inner()).unwrap()
}

/// Copy any output to the clipboard in an OS agnostic way
pub fn copy_to_clipboard(string: impl AsRef<str>) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(string.as_ref().to_string()).unwrap();
}
