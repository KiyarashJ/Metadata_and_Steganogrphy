pub fn read_file(carrier: std::path::PathBuf) -> String {
    let read_file = std::fs::read(carrier).unwrap();
    let carrier = String::from_utf8_lossy(&read_file).into_owned();
    carrier
}