#[macro_export]
/// Copies a string, with formatting, to the systems clipboard
/// It is unsafe, because it needs to access a global instance of the state
macro_rules! copy {
    ($($arg:tt)+) => {
        use crate::utils::logger::{STATE, copy_to_clipboard};
        use log::debug;
        unsafe {
            if STATE.should_copy {
                let text = format!($($arg)+);
                if !text.is_empty() {
                    debug!("Copied output to buffer");
                    copy_to_clipboard(text);
                } else {
                    debug!("Nothing to copy");
                }
            }
        }
    };
}
