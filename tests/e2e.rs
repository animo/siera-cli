#[macro_use]
mod macros;

mod utils;

use agent::modules::connection::Connection;
use regex::Regex;
use speculoos::prelude::*;
use utils::helpers::{run_test, REGEX_UUID};

test! { smoke |cli| {
    let re = Regex::new(r"^agent-cli \d\.\d.\d").unwrap();
    let ret = agent!(cli, "--version");
    assert_that(&ret).matches(|v| re.is_match(v))
}}

test! { create_a_connection_and_list_connections |cli| {
    let connections = agent!(cli, "connection list");
    assert_that(&connections).is_equal_to(String::from("[]"));
    agent!(cli, "connection invite");
    let connections = agent!(cli, "connection list");
    let connections: Vec<Connection> = serde_json::from_str(&connections).unwrap();
    assert_that(&connections).has_length(1);
}}

test! { create_connection_and_send_a_message |cli| {
    agent!(cli, "connection invite");
    let connections_str = agent!(cli, "connection list");
    let connection: Vec<Connection> = serde_json::from_str(&connections_str).unwrap();
    let result = agent!(cli, "message --connection-id={} --message={}", connection[0].connection_id, "bar");
    assert_that(&result).is_equal_to(String::from(""))
}}

test! { create_invitation_and_receive_invitation |cli| {
    let invitation_str = agent!(cli, "connection invite");
    let list = invitation_str.split('\n').collect::<Vec<&str>>();
    let url = list.get(1).unwrap();
    let result = agent!(cli, "connection receive --url={}", *url);
    let re = Regex::new(REGEX_UUID).unwrap();
    assert_that(&result).matches(|v| re.is_match(v))
}}
