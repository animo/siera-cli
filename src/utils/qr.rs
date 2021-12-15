/// Parse `Invitation to a `qr_code`
pub fn print_qr_code(text: &str) {
    qr2term::print_qr(text).expect("Error printing QR!");
}
