use clipboard::{ClipboardContext, ClipboardProvider};
use log::LevelFilter;
use serde::Serialize;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

static mut LOGGER_INITED: bool = false;

pub fn init(level: LevelFilter) {
    unsafe {
        if LOGGER_INITED {
            panic!("Logger should only be initialized once!");
        }
    }
    let _ = TermLogger::init(
        level,
        ConfigBuilder::new()
            .set_location_level(LevelFilter::Debug)
            .set_thread_level(LevelFilter::Off)
            .set_time_level(LevelFilter::Debug)
            .set_target_level(LevelFilter::Debug)
            .set_max_level(LevelFilter::Off)
            .build(),
        TerminalMode::default(),
        ColorChoice::Never,
    );
    unsafe {
        LOGGER_INITED = true;
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::debug!($($arg)*);
    };
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        $crate::log::error!($($arg)*);
    };
}

pub fn pretty_stringify_obj(obj: impl Serialize) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

    obj.serialize(&mut ser).unwrap();

    String::from_utf8(ser.into_inner()).unwrap()
}

pub fn pretty_print_obj(obj: impl Serialize) {
    println!("{}", pretty_stringify_obj(obj));
}

pub fn copy_to_buffer(string: impl AsRef<str>) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(string.as_ref().to_string()).unwrap();
}
