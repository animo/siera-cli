pub enum CliError {
    InvalidConfigurationPath,
    InvalidEnvironment,
    NoEndpointSupplied,
}

pub fn get_cli_error_string(error: CliError) -> &'static str {
    match error {
        CliError::InvalidConfigurationPath => "Invalid configuration path",
        CliError::InvalidEnvironment => "Invalid environment",
        CliError::NoEndpointSupplied => {
            "No endpoint supplied. Supply an endpoint either via `--endpoint` or via `--config`"
        }
    }
}
