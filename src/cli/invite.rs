use crate::agent;
use crate::typing;
use crate::utils::logger::Log;
use crate::utils::qr;

pub async fn run(agent: agent::Agent, config: typing::InviteConfiguration<'_>) {
    let invitation = agent.create_invitation(&config).await;

    if config.qr {
        qr::print_qr_code(&invitation.invitation_url);
    } else {
        Log::output(&invitation.invitation_url);
    }
}
