mod cli;
mod modules;
mod utils;

use agent::agent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use clap::StructOpt;
use cli::{Cli, Commands};
use modules::connections::ConnectionSubcommands;
use utils::logger::Log;

fn main() {
    let cli = Cli::parse();

    let _agent = CloudAgentPython::new("jes", Some("yes"), CloudAgentPythonVersion::ZeroSixZero);

    match &cli.commands {
        Commands::Connections(options) => match &options.commands {
            ConnectionSubcommands::Invite {
                auto_accept,
                multi_use,
                qr,
                toolbox,
                alias,
            } => {
                Log::default().log(format!("{:?}", auto_accept));
                Log::default().log(format!("{:?}", multi_use));
                Log::default().log(format!("{:?}", qr));
                Log::default().log(format!("{:?}", toolbox));
                Log::default().log(format!("{:?}", alias));
            }
        },
        Commands::Features(_) => Log::default().log("Features!"),
    }
}
