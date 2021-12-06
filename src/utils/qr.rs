use crate::error;

// Parse `Invitation to a `qr_code`
pub fn print_qr_code(text: &String) {
    if let Err(_) = qr2term::print_qr(text) {
        error::throw(error::Error::CannotCreateQrCode)
    }
}
