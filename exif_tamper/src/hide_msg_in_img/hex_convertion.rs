pub fn to_hex(st: &[u8]) -> String {
    hex::encode(st)
}

pub fn from_hex(hex: &str) -> Vec<u8> {
    hex::decode(hex).expect("Failed to convert from hex")   
}