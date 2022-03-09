use std::convert::From;

/// Help documentation for CLI commands.

pub enum HelpStrings {
    Cli,
    Endpoint,
    ApiKey,
    Copy,
    Quiet,
    Verbose,
    Config,
    Environment,
}

impl From<HelpStrings> for Option<&str> {
    fn from(help_string: HelpStrings) -> Option<&'static str> {
        Some(help_string.as_str())
    }
}

impl HelpStrings {
    fn as_str(&self) -> &'static str {
        match self {
            HelpStrings::Cli => HELP_STRING_CLI,
            HelpStrings::Endpoint => "The Aries agent endpoint requests are sent to",
            HelpStrings::ApiKey => "This API key will be passed to the agent.",
            HelpStrings::Copy => "Copy output to your clipboard.",
            HelpStrings::Quiet => "Suppresses most output.",
            HelpStrings::Verbose => "Print debug logs.",
            HelpStrings::Config => "Path to your configuration file.",
            HelpStrings::Environment => "Specify your current environment.",
            // TODO: Add docs for all subcommands, e.g., ConnectionsAll.
        }
    }
}

const HELP_STRING_CLI: &str = "
--- Aries cli --- 

To begin working with the aries-cli, run the following command:

    $ aries-cli configuration initialize

This command will initialize the configuration file and makes sure
that you do not have to pass the --endpoint argument with every call.

Some example commands are the following:

    $ aries-cli connections 
        - fetches all the connections (jq compatible)
    $ aries-cli connections invite -qr
        - create an invitation (as a qr code)
    $ aries-cli features   
        - Fetches all the features of the cloudagent
    $ aries-cli schema create --name FOO -a BAR -a BAZ 
        - Create a new schema with the name as 'FOO' and the attributes as 'BAR' and 'BAZ'

-----------------
";
