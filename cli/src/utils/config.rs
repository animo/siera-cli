use std::path::Path;

use ini::{Ini, Properties};

use crate::error;
use crate::error::Result;

/// Load a config file and ignore errors as we will just fall back on the option provided
fn load(path: &Path) -> Result<Ini> {
    Ini::load_from_file(path).map_err(|_| error::Error::InvalidConfigurationPath.into())
}

/// Get a section in the config file
fn get_section(key: &str, cfg: &Ini) -> Result<Properties> {
    cfg.section(Some(key))
        .map(|s| s.to_owned())
        .ok_or_else(|| error::Error::InvalidEnvironment.into())
}

/// Get a value by key in the section
fn get_value_by_key(key: &str, prop: &Properties) -> Result<String> {
    prop.get(key)
        .map(String::from)
        .ok_or_else(|| error::Error::NoConfigKey.into())
}

/// Get a value by path, section and key
pub fn get_value_from_config(
    path: &Path,
    section: impl AsRef<str>,
    key: impl AsRef<str>,
) -> Result<String> {
    let cfg = load(path)?;

    let sec = get_section(section.as_ref(), &cfg)?;

    get_value_by_key(key.as_ref(), &sec)
}
