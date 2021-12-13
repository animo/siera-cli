use super::register::Module;
use crate::agent::agents::Agent;
use crate::typing::InvitationConfig;
use crate::utils::logger::Log;
use crate::utils::qr;
use async_trait::async_trait;
use clap::ArgMatches;

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
            Log::log(&invitation.invitation_url);
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
