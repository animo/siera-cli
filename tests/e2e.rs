#![cfg(test)]

use std::env;
use speculoos::prelude::*;
use regex::Regex;
use std::process::Command;

fn getAgentUrl() -> String {
  match env::var("AGENT_URL") {
    Ok(v) => v,
    Err(_) => String::from("https://agent.community.animo.id"),
  }
}

#[test]
fn smoke() -> () {
  let re = Regex::new(r"^agent-cli \d\.\d.\d").unwrap();
  assert_that(&aries_cli("--version")).matches(|v| re.is_match(v))
}

fn aries_cli(cmd: &str) -> String {
  let result = Command::new("cargo")
    .arg("run")
    .arg("-q")
    .arg("--")
    .arg(cmd)
    .output();
  assert!(result.is_ok(), "result of {} was not OK", cmd);
  String::from_utf8(result.unwrap().stdout).unwrap()
}
