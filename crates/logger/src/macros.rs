/// Copies a string, with formatting, to the systems clipboard
/// It is unsafe, because it needs to access a global instance of the state
#[macro_export]
macro_rules! copy {
    ($($arg:tt)+) => {
            if crate::logger::STATE.read().unwrap().should_copy {
                let text = format!($($arg)+);
                if !text.is_empty() {
                    log_debug!("Copied output to buffer");
                    crate::logger::copy_to_clipboard(text);
                } else {
                    log_debug!("Nothing to copy");
                }
        }
    };
}

/// Simple info logger
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {
        if crate::logger::STATE.read().unwrap().level == crate::logger::LogLevel::Info {
            println!($($arg)+);
        }
    };
}

/// Simple debug logger
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => {
        if crate::logger::STATE.read().unwrap().level == crate::logger::LogLevel::Debug {
            println!("[DEBUG]: {}", format!($($arg)+));
        }
    };
}

/// Simple trace logger
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)+) => {
        if crate::logger::STATE.read().unwrap().level == crate::logger::LogLevel::Trace {
            println!("[TRACE]: {}", format!($($arg)+));
        }
    };
}

/// Simple warning logger
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => {
        if crate::logger::STATE.read().unwrap().level == crate::logger::LogLevel::Warn {
            println!("[TRACE]: {}", format!($($arg)+));
            println!("[WARN]: {}", format!($($arg)+));
        }
    };
}

/// Simple error logger
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {
        if crate::logger::STATE.read().unwrap().level == crate::logger::LogLevel::Error{
            println!("[ERROR]: {}", format!($($arg)+));
        }
    };
}
