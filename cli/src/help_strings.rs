/// Help documentation for CLI commands.
pub mod top_level {
  pub static Endpoint: &str = "The Aries agent endpoint requests are sent to";

  pub static ApiKey: &str = "This API key will be passed to the agent.";

  pub static Copy: &str = "Copy output to your clipboard.";

  pub static Quiet: &str = "Suppresses most output.";

  pub static Verbose: &str = "Print debug logs. Cannot be used with --quiet.";

  pub static Config: &str = "Path to your configuration file.";

  pub static Environment: &str = "Specify your current environment.";
}

// TODO: Add docs for all subcommands in their own modules.
