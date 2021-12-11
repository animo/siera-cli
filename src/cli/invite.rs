use crate::agent::agents::Agent;
use crate::typing::InvitationConfig;
use crate::utils::logger::Log;
use crate::utils::qr;

pub async fn run(agent: &dyn Agent, config: InvitationConfig<'_>) {
    let invitation = agent.create_invitation(&config).await;

    if config.qr {
        qr::print_qr_code(&invitation.invitation_url);
    } else {
        Log::log(&invitation.invitation_url);
    }
}
