/// Copies a string, with formatting, to the systems clipboard
#[macro_export]
macro_rules! copy {
    ($($arg:tt)+) => {
            if ::siera_logger::STATE.read().unwrap().should_copy_relevant {
                let text = format!($($arg)+);
                if !text.is_empty() {
                    debug!({"message": "Copied output to buffer"});
                    ::siera_logger::copy_to_clipboard(text);
                } else {
                    debug!({"message": "Nothing to copy"});
                }
        }
    };
}

/// Simple wrapper around eprint!
#[macro_export]
macro_rules! elog {
    ($($arg:tt)*) => {
        if ::siera_logger::STATE.read().unwrap().level != ::siera_logger::LogLevel::Off {
            eprint!($($arg)*);
        }
    };
}

/// Generic logger. Should not be used outside of this file
#[macro_export]
macro_rules! internal_log {
    ($level:expr, $($arg:tt)+) => {
        if ::siera_logger::STATE.read().unwrap().level >= $level {
            let value: $crate::serde_json::Value = $crate::serde_json::json!($($arg)+);
            if ::siera_logger::STATE.read().unwrap().should_output_json {
                let value = match value {
                    $crate::serde_json::Value::Object(o) => {
                        let mut o = o.clone();
                        o.insert("level".to_string(), $crate::serde_json::Value::String($level.to_string_without_color()));
                        $crate::serde_json::Value::Object(o)
                    },
                    v => v,
                };
                println!("{}", $crate::serde_json::to_string_pretty(&value).unwrap());
            } else {
                match value {
                    $crate::serde_json::Value::Object(o) => {
                        let values = o.values();
                        for value in values {
                            if let Some(value) = value.as_str() {
                                println!("[{}] {}", $level.to_string_with_color(), value);
                            }
                        }
                    },
                    _ => (),
                };
            }
        }
    };
}

/// Simple info logger
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Info, $($arg)+)
    }
}

/// Simple debug logger
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Debug, $($arg)+);
    };
}

/// Simple trace logger
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Trace, $($arg)+);
    };
}

/// Simple warning logger
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Warn, $($arg)+);
    };
}

/// Simple error logger
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Error, $($arg)+);
    };
}
