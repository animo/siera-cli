#![cfg(test)]

use regex::Regex;
use speculoos::prelude::*;

use agent::modules::connection::Connection;

mod helpers;
use helpers::run_test;

#[tokio::test]
async fn smoke() -> () {
    run_test(|agent_cli| {
        let re = Regex::new(r"^agent-cli \d\.\d.\d").unwrap();
        assert_that(&agent_cli.exec(&["--version"])).matches(|v| re.is_match(v))
    })
    .await
}

#[tokio::test]
async fn create_a_connection_and_list_connections() -> () {
    run_test(|agent_cli| {
        let connections = agent_cli.exec(&["connection", "list"]);
        assert_that(&connections).is_equal_to(&String::from("[]"));
        let _ = agent_cli.exec(&["connection", "invite", "--alias", "test"]);
        let connections_str = agent_cli.exec(&["connection", "list"]);
        let connections: Vec<Connection> = serde_json::from_str(&connections_str).unwrap();
        assert_that(&connections).has_length(1);
    })
    .await;
}

#[tokio::test]
async fn create_a_connection_and_send_a_message() -> () {
    run_test(|agent_cli| {
        let _ = agent_cli.exec(&["connection", "invite", "--alias", "foo"]);
        let connections_str = agent_cli.exec(&["connection", "list"]);
        let connection: Vec<Connection> = serde_json::from_str(&connections_str).unwrap();
        let result = agent_cli.exec(&[
            "message",
            "--connection-id",
            &connection[0].connection_id,
            "--message=BAR",
        ]);
        assert_that(&result).is_equal_to(String::from(""))
    })
    .await
}
