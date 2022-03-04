use std::convert::From;

/// Help documentation for CLI commands.

pub enum HelpStrings {
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
      HelpStrings::Endpoint => "The Aries agent endpoint requests are sent to",
      HelpStrings::ApiKey => "This API key will be passed to the agent.",
      HelpStrings::Copy => "Copy output to your clipboard.",
      HelpStrings::Quiet => "Suppresses most output.",
      HelpStrings::Verbose => "Print debug logs.",
      HelpStrings::Config => "Path to your configuration file.",
      HelpStrings::Environment => "Specify your current environment.",
      // TODO: Add docs for all subcommands in their own modules.
    }
  }
}

