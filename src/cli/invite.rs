use crate::agent::agent::Agent;
use crate::typing;
use crate::utils::logger::Log;
use crate::utils::qr;
use crate::HttpAgent;

pub async fn run(agent: HttpAgent, config: typing::InviteConfiguration<'_>) {
    let invitation = agent.create_invitation(&config).await;

    if config.qr {
        qr::print_qr_code(&invitation.invitation_url);
    } else {
        Log::log(&invitation.invitation_url);
    }
}
