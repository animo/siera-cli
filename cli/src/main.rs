mod utils;

use utils::logger::Log;

use agent::agent_python::{CloudAgentPython, CloudAgentPythonVersion};

fn main() {
    let agent = CloudAgentPython::new("yes", Some("yes"), CloudAgentPythonVersion::ZeroSixZero);
    let logger = Log {
        should_copy: false,
        suppress_output: false,
    };
    logger.log(format!("{:?}", agent));
    logger.log_list(vec!["test"]);
    logger.log_pretty(vec!["test"]);
    logger.error("ERROR")
}
