#![cfg(test)]

use std::io;
use std::process::{Command, Output};

#[test]
fn smoke() -> () {
  let result = aries_cli("--version");
  assert_eq!(version, "agent-cli 0.2.0\n")
}

fn aries_cli(cmd: &str) -> String {
  let result = Command::new("cargo")
    .arg("run")
    .arg("-q")
    .arg("--")
    .arg(cmd)
    .output();
  assert!(result.is_ok(), format!("result of {} was not OK", cmd));
  String::from_utf8(result.unwrap().stdout).unwrap()
}
