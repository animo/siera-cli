use super::register::Module;
use crate::agent::agents::Agent;
use crate::utils::qr;
use async_trait::async_trait;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Type of the invitation configuration as received by the cli
pub struct InvitationConfig {
    /// Whether the invitation should auto accept
    pub auto_accept: bool,

    /// Whether the invitation should be multi use
    pub multi_use: bool,

    /// Alias for the connection that will be created with that invitation
    pub alias: Option<String>,

    /// Whether it will print a qr code instead of a url
    pub qr: bool,

    /// Whether it should use a pre-configured toolbox configuration
    pub toolbox: bool,
}

/// Type of the received invitation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    /// Connection id
    pub connection_id: String,

    /// Invitation object
    pub invitation: Map<String, Value>,

    /// Invitation url that can be used to accept it by another party
    pub invitation_url: String,

    /// Alias for the given invitation
    pub alias: Option<String>,
}

/// Invitations module for the agent
pub struct InvitationsModule;

/// Implementation of a module for invitations
#[async_trait(?Send)]
impl Module<InvitationConfig> for InvitationsModule {
    async fn run(agent: &dyn Agent, config: InvitationConfig) {
        let invitation = agent.create_invitation(&config).await;

        if config.qr {
            qr::print_qr_code(&invitation.invitation_url);
        } else {
            agent.logger().log(&invitation.invitation_url);
        }
    }

    async fn register<'a>(agent: &dyn Agent, matches: &ArgMatches<'a>) {
        if let Some(matches_invite) = matches.subcommand_matches("invite") {
            let auto_accept = matches_invite.is_present("auto-accept");
            let multi_use = matches_invite.is_present("multi-use");
            let qr = matches_invite.is_present("qr");
            let toolbox = matches_invite.is_present("toolbox");
            let alias = matches_invite
                .value_of("alias")
                .map(|alias| alias.to_string());

            let config = InvitationConfig {
                auto_accept,
                multi_use,
                alias,
                qr,
                toolbox,
            };

            InvitationsModule::run(agent, config).await;
        }
    }
}
