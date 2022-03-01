/// Help documentation for CLI commands.
pub mod top_level {
  pub const ENDPOINT: &'static str = "The Aries agent endpoint requests are sent to";

  pub const API_KEY: &'static str = "This API key will be passed to the agent.";

  pub const COPY: &'static str = "Copy output to your clipboard.";

  pub const QUIET: &'static str = "Suppresses most output.";

  pub const VERBOSE: &'static str = "Print debug logs. Cannot be used with --quiet.";

  pub const CONFIG: &'static str = "Path to your configuration file.";

  pub const ENVIRONMENT: &'static str = "Specify your current environment.";
}

// TODO: Add docs for all subcommands in their own modules.
