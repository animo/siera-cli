use ini::{Ini, Properties};
use std::env;

/// Load a config file and ignore errors as we will just fall back on the option provided
fn load(path: &str) -> Option<Ini> {
    let cfg = Ini::load_from_file(path);

    match cfg {
        Ok(c) => Some(c),
        _ => None,
    }
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
pub fn get_value(path: &str, section: &str, key: &str) -> Option<String> {
    let home = env::var("HOME");

    let full_path: Option<String> = match home {
        Ok(mut h) => {
            h.push_str(path);
            Some(h)
        }
        Err(_) => None,
    };

    let cfg = match full_path {
        Some(f) => load(f.as_str()),
        None => None,
    };

    let sec = match cfg {
        Some(c) => get_section(section, &c),
        None => None,
    };

    match sec {
        Some(c) => get_value_by_key(key, &c),
        None => None,
    }
}
