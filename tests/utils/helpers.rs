use serde::{Deserialize, Serialize};
use siera_agent::modules::multitenancy::MultitenancyCreateResponse;
use std::env;
use std::panic;
use std::process::Command;

pub const REGEX_UUID: &str =
    r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$";

/// Helper function which does test set up and teardown
pub async fn run_test<T>(test: T)
where
    T: FnOnce(&TestAgentCli) + panic::UnwindSafe,
{
    let (mut siera_cli, wallet_id) = setup().await;
    let result = panic::catch_unwind(|| test(&siera_cli));
    teardown(&mut siera_cli, wallet_id);
    assert!(result.is_ok(), "Test execution failed")
}

fn get_agent_url() -> String {
    env::var("AGENT_URL").unwrap_or_else(|_| String::from("http://localhost:8010"))
}

#[derive(Serialize, Deserialize)]
struct MultiTenancyCreateResponseWrapper {
    response: MultitenancyCreateResponse,
}

async fn setup() -> (TestAgentCli, String) {
    let mut siera_cli = TestAgentCli::new(None);
    let response_str = siera_cli.exec("multitenancy create");
    let wallet_response: MultiTenancyCreateResponseWrapper =
        serde_json::from_str(&response_str).unwrap();
    siera_cli.scope_to_wallet(wallet_response.response.token);
    (siera_cli, wallet_response.response.wallet_id)
}

fn teardown(siera_cli: &mut TestAgentCli, wallet_id: String) {
    siera_cli.unscope_from_wallet();
    siera_cli.exec(&format!("multitenancy remove --wallet-id={wallet_id}"));
}

/// A test utility that wraps common args we want to pass to every command
/// we give to the agent as well as handling of process stdout and stderr.
pub struct TestAgentCli {
    wallet_token: Option<String>,
}

impl TestAgentCli {
    pub fn new(token: Option<String>) -> Self {
        TestAgentCli {
            wallet_token: token,
        }
    }

    pub fn scope_to_wallet(&mut self, token: String) {
        self.wallet_token = Some(token);
    }

    pub fn unscope_from_wallet(&mut self) {
        self.wallet_token = None;
    }

    pub fn exec(&self, command: &str) -> String {
        let agent_url = get_agent_url();
        let mut agent_args = format!("--agent-url={agent_url} ");
        match &self.wallet_token {
            Some(token) => agent_args.push_str(&format!("--token={token} ")),
            None => (),
        }
        agent_args.push_str(command);
        let result = Command::new("cargo")
            .args(["run", "--package=siera", "--quiet", "--", "-j"])
            .args(agent_args.split(' ').collect::<Vec<&str>>())
            .output();
        let output = result
            .map_err(|e| format!("Command failed '{agent_args:?}' with '{e}'"))
            .unwrap();
        if !output.status.success() {
            println!();
            println!("=============================");
            println!("Command failed: {agent_args:?}");
            println!("[STDERR]: {}", String::from_utf8_lossy(&output.stderr));
            println!("[STDOUT]: {}", String::from_utf8_lossy(&output.stdout));
            println!("=============================");
            println!();
            panic!("Test failed!");
        }
        let string_output = String::from_utf8(output.stdout).unwrap();
        String::from(string_output.trim())
    }
}
