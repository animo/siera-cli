#[macro_use]
mod macros;

mod utils;

use siera_agent::modules::connection::Connection;
use regex::Regex;
use speculoos::prelude::*;
use utils::helpers::{run_test, REGEX_UUID};

test! { smoke |cli| {
    let re = Regex::new(r"^siera \d\.\d.\d").unwrap();
    let ret = siera!(cli, "--version");
    assert_that(&ret).matches(|v| re.is_match(v))
}}

test! { create_a_connection_and_list_connections |cli| {
    let connections = siera!(cli, "connection list");
    assert_that(&connections).is_equal_to(String::from("[]"));
    siera!(cli, "connection invite");
    let connections = siera!(cli, "connection list");
    let connections: Vec<Connection> = serde_json::from_str(&connections).unwrap();
    assert_that(&connections).has_length(1);
}}

test! { create_connection_and_send_a_message |cli| {
    siera!(cli, "connection invite");
    let connections_str = siera!(cli, "connection list");
    let connection: Vec<Connection> = serde_json::from_str(&connections_str).unwrap();
    let result = siera!(cli, "message --connection-id={} --message={}", connection[0].id, "bar");
    assert_that(&result).is_equal_to(String::from("Successfully sent message"))
}}

test! { create_invitation_and_receive_invitation |cli| {
    let invitation_str = siera!(cli, "connection invite");
    let list = invitation_str.split('\n').collect::<Vec<&str>>();
    let url = list.get(1).unwrap();
    let result = siera!(cli, "connection receive --url={}", *url);
    let re = Regex::new(REGEX_UUID).unwrap();
    assert_that(&result).matches(|v| re.is_match(v))
}}
