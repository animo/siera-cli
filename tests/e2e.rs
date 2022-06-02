#![cfg(test)]

use std::env;
use std::panic;
use std::process::Command;

use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use speculoos::prelude::*;

use agent::modules::connection::Connection;

#[tokio::test]
async fn smoke() -> () {
    run_test(|agent_cli| {
        let re = Regex::new(r"^agent-cli \d\.\d.\d").unwrap();
        assert_that(&agent_cli.exec(&["--version"])).matches(|v| re.is_match(v))
    })
    .await
}

#[tokio::test]
async fn create_and_list_connections() -> () {
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
async fn sends_a_message() -> () {
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

/// Helper function which does test set up and teardown
async fn run_test<T>(test: T) -> ()
where
    T: FnOnce(TestAgentCli) -> () + panic::UnwindSafe,
{
    let (agent_cli, wallet_id) = setup().await;
    let result = panic::catch_unwind(|| test(agent_cli));
    teardown(wallet_id).await;
    assert!(result.is_ok())
}

fn get_agent_url() -> String {
    match env::var("AGENT_URL") {
        Ok(v) => v,
        Err(_) => String::from("http://localhost:8010"),
    }
}

/// TODO: This struct should be moved because this functionality should be
/// provided by the CLI
#[derive(Debug, Default, Serialize, Deserialize)]
struct CreateWalletResponse {
    token: String,
    wallet_id: String,
}

/// TODO: Rework this once we have the ability to create a sub-wallet from the CLI.
/// We should just be using the CLI directly.
async fn setup() -> (TestAgentCli, String) {
    let body: Value = serde_json::from_str(
        r#"{
             "image_url": "https://aries.ca/images/sample.png",
             "key_management_mode": "managed",
             "label": "Alice",
             "wallet_dispatch_type": "default",
             "wallet_key": "MySecretKey123",
             "wallet_name": "MyCoolName",
             "wallet_type": "indy",
             "wallet_webhook_urls": [
                 "http://localhost:8022/webhooks"
              ]
          }"#,
    )
    .unwrap();
    let url = format!("{}/multitenancy/wallet", get_agent_url());
    let client = Client::new()
        .post(url)
        .header("content-type", "application/json")
        .json(&body);
    let json = match client.send().await {
        Ok(res) => res.json::<CreateWalletResponse>().await.unwrap(),
        Err(e) => panic!("Setup failed {}", e),
    };
    (TestAgentCli::new(json.token), json.wallet_id)
}

/// TODO: Rework this once we have the ability to create a sub-wallet from the CLI.
/// We should just be using the CLI directly.
async fn teardown(wallet_id: String) {
    let url = format!(
        "{}/multitenancy/wallet/{}/remove",
        get_agent_url(),
        wallet_id
    );
    let client = Client::new()
        .post(url)
        .header("content-type", "application/json");
    let res = match client.send().await {
        Ok(res) => res,
        Err(e) => panic!("Setup failed {}", e),
    };

    assert!(res.status().as_u16() < 399, "bad status for wallet removal")
}

/// A test utility that wraps common args we want to pass to every command
/// we give to the agent as well as handling of process stdout and stderr.
struct TestAgentCli {
    token: String,
}

impl TestAgentCli {
    pub fn new(token: String) -> Self {
        TestAgentCli { token }
    }

    pub fn exec(&self, args: &[&str]) -> String {
        let agent_url = get_agent_url();
        let mut all_args = vec![
            "run",
            "--quiet",
            "--",
            "--agent-url",
            &agent_url,
            "--token",
            &self.token,
        ];
        all_args.extend(args.to_vec());
        let result = Command::new("cargo").args(&all_args).output();
        let output = match result {
            Ok(o) => o,
            Err(e) => panic!("Command failed \"{:?}\" with \"{}\"", &all_args, e),
        };
        if !output.status.success() {
            println!();
            println!("=============================");
            println!("Command failed: {:?}", &all_args);
            println!("{}", String::from_utf8_lossy(&output.stderr));
            println!("=============================");
            println!();
            panic!("Test failed!");
        }
        let string_output = String::from_utf8(output.stdout).unwrap();
        String::from(string_output.trim())
    }
}
