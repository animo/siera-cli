use std::path::Path;

use ini::{Ini, Properties};

use crate::error::{get_cli_error_string, CliError};

use super::logger::Log;

// Convert to toml

/// Load a config file and ignore errors as we will just fall back on the option provided
fn load(path: &Path) -> Option<Ini> {
    Ini::load_from_file(path).ok()
}

/// Get a section in the config file
fn get_section(key: &str, cfg: &Ini) -> Option<Properties> {
    cfg.section(Some(key)).map(|s| s.to_owned())
}

/// Get a value by key in the section
fn get_value_by_key(key: &str, prop: &Properties) -> Option<String> {
    prop.get(key).map(String::from)
}

/// Get a value by path, section and key
pub fn get_value_from_config(
    path: &Path,
    section: impl AsRef<str>,
    key: impl AsRef<str>,
) -> Option<String> {
    let cfg = load(path);
    let logger = Log::default();

    let sec = match cfg {
        Some(c) => get_section(section.as_ref(), &c),
        None => logger.error(get_cli_error_string(CliError::InvalidConfigurationPath)),
    };

    match sec {
        Some(c) => get_value_by_key(key.as_ref(), &c),
        None => logger.error(get_cli_error_string(CliError::InvalidEnvironment)),
    }
}
