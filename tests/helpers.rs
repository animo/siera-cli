use agent::modules::multitenancy::MultitenancyCreateResponse;
use std::env;
use std::panic;
use std::process::Command;

/// Helper function which does test set up and teardown
pub async fn run_test<T>(test: T) -> ()
where
    T: FnOnce(&TestAgentCli) -> () + panic::UnwindSafe,
{
    let (mut agent_cli, wallet_id) = setup().await;
    let result = panic::catch_unwind(|| test(&agent_cli));
    teardown(&mut agent_cli, wallet_id);
    assert!(result.is_ok(), "Test execution failed")
}

fn get_agent_url() -> String {
    env::var("AGENT_URL").unwrap_or_else(|_| String::from("http://localhost:8010"))
}

async fn setup() -> (TestAgentCli, String) {
    let mut agent_cli = TestAgentCli::new(None);
    let response_str = agent_cli.exec(&["multitenancy", "create"]);
    let wallet_response: MultitenancyCreateResponse = serde_json::from_str(&response_str).unwrap();
    agent_cli.scope_to_wallet(wallet_response.token);
    (agent_cli, wallet_response.wallet_id)
}

fn teardown(agent_cli: &mut TestAgentCli, wallet_id: String) {
    agent_cli.unscope_from_wallet();
    agent_cli.exec(&["multitenancy", "remove", "--wallet-id", &wallet_id]);
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

    pub fn exec(&self, args: &[&str]) -> String {
        let agent_url = get_agent_url();
        let mut agent_args = vec!["--agent-url", &agent_url];
        match &self.wallet_token {
            Some(token) => agent_args.extend(vec!["--token", &token]),
            None => (),
        }
        agent_args.extend(&args.to_vec());
        let result = Command::new("cargo")
            .args(vec!["run", "--quiet", "--"])
            .args(&agent_args)
            .output();
        let output = match result {
            Ok(o) => o,
            Err(e) => panic!("Command failed \"{:?}\" with \"{}\"", &agent_args, e),
        };
        if !output.status.success() {
            println!();
            println!("=============================");
            println!("Command failed: {:?}", &agent_args);
            println!("{}", String::from_utf8_lossy(&output.stderr));
            println!("=============================");
            println!();
            panic!("Test failed!");
        }
        let string_output = String::from_utf8(output.stdout).unwrap();
        String::from(string_output.trim())
    }
}
