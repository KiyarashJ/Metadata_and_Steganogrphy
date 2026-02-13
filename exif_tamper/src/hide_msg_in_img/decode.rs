use anyhide::{decode};
use crate::hide_msg_in_img::{read_file_name::read_file, hex_convertion::from_hex};
use x25519_dalek::StaticSecret;
use crate::hide_msg_in_img::write_on_file::{Res, writing};


pub fn decode_img(cr: &std::path::PathBuf, passphrase: &str, code: &str, secret: String) -> &'static str {
    
    let cleaned = secret.trim();
    if cleaned.len() != 64 {
        panic!("Secret key must be exactly 64 hex characters (32 bytes), now : {}", cleaned.len());
    }

    
    let decoded_secret = from_hex(cleaned); 

    
    let mut secret_as_bytes = [0u8; 32];
    secret_as_bytes.copy_from_slice(&decoded_secret);

    
    let key = StaticSecret::from(secret_as_bytes);

    let carrier = read_file(cr.to_owned());
    let decoding = decode(code, &carrier, passphrase, &key);
    println!("msg : {}", String::from_utf8_lossy(decoding.message.as_bytes()));

    
    let _ = writing(Res::Decode(decoding), "decoded_message", "txt");
    "done"
}

