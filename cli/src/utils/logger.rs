use clipboard::{ClipboardContext, ClipboardProvider};
use log::{info, LevelFilter};
use serde::Serialize;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

#[derive(Default)]
pub struct LoggerState {
    pub init: bool,
    pub should_copy: bool,
}

pub static mut STATE: LoggerState = LoggerState {
    init: false,
    should_copy: false,
};

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
            .add_filter_allow(String::from("aries_cli"))
            .add_filter_allow(String::from("cloudagent_"))
            .add_filter_allow(String::from("workflow"))
            .build(),
        TerminalMode::default(),
        ColorChoice::Never,
    );
    unsafe {
        STATE.init = true;
    }
}

pub fn pretty_stringify_obj(obj: impl Serialize) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

    obj.serialize(&mut ser).unwrap();

    String::from_utf8(ser.into_inner()).unwrap()
}

pub fn copy_to_clipboard(string: impl AsRef<str>) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(string.as_ref().to_string()).unwrap();
}
