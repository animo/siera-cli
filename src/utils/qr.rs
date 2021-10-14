use crate::{error, typing};

// Parse `Invitation to a `qr_code`
pub fn print_invitation_and_qr_for_invitation(invitation: &typing::Invitation) {
    // Can use unwrap here because the url is supplied by the endpoint
    let url = reqwest::Url::parse(&invitation.invitation_url.to_string()).unwrap();

    if let Err(_) = qr2term::print_qr(&url.as_ref()) {
        error::throw(error::Error::CannotCreateQrCode)
    }
}
