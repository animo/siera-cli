/// Parse any string to a qr code
pub fn print_qr_code(text: impl AsRef<str>) -> Result<(), String> {
    qr2term::print_qr(text.as_ref()).map_err(|e| e.to_string())
}
