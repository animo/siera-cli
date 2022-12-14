/// Copies a string, with formatting, to the systems clipboard
#[macro_export]
macro_rules! copy {
    ($($arg:tt)+) => {
            if ::siera_logger::STATE.read().unwrap().should_copy {
                let text = format!($($arg)+);
                if !text.is_empty() {
                    log_debug!("Copied output to buffer");
                    ::siera_logger::copy_to_clipboard(text);
                } else {
                    log_debug!("Nothing to copy");
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

/// Simple wrapper around println!
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        if !vec![::siera_logger::LogLevel::Off, ::siera_logger::LogLevel::Json].contains(&::siera_logger::STATE.read().unwrap().level) {
            println!($($arg)*);
        }
    };
}

/// Generic logger. Should not be used outside of this file
#[macro_export]
macro_rules! internal_log {
    ($level:expr, $($arg:tt)+) => {
        if ::siera_logger::STATE.read().unwrap().level >= $level {
            log!("[{}] {}", $level, format!($($arg)+));
        }
    };
}

/// Simple info logger
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Info, $($arg)+);
    };
}

/// Simple debug logger
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Debug, $($arg)+);
    };
}

/// Simple Json logger
#[macro_export]
macro_rules! log_json {
    ($($arg:tt)+) => {
        if ::siera_logger::LogLevel::Json ==
            ::siera_logger::STATE.read().unwrap().level
        {
            println!($($arg)*);
        }
    };
}

/// Simple trace logger
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Trace, $($arg)+);
    };
}

/// Simple warning logger
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Warn, $($arg)+);
    };
}

/// Simple error logger
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {
        internal_log!(::siera_logger::LogLevel::Error, $($arg)+);
    };
}
