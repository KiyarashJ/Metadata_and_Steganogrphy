use std::io::Write;
use anyhide::{decoder::DecodedMessage};
use crate::hide_msg_in_img::encode::EncodeResult;
use hex::encode as hex_encode;

pub enum Res {
    Encode(EncodeResult),
    Decode(DecodedMessage)
}


pub fn writing(info: Res, filename: &str, ext: &str) {
    let path_buffer = std::path::PathBuf::from(format!("{}.{}", filename, ext));
    let mut file_create = std::fs::File::create(path_buffer).unwrap();
    match info {
        Res::Encode(encode) => {
            let _ = writeln!(file_create, "code: {}", encode.code); 
            let _ = writeln!(file_create, "public_key: {}", hex_encode(&encode.public_key.as_bytes()));
            let _ = writeln!(file_create, "secret_key: {}", hex_encode(&encode.secret_key.to_bytes()));
        },
        
        Res::Decode(decode) => {
                    let message_str = decode.message;
                    let _ = writeln!(file_create, "message: {}", message_str);
                    let fragments_str = decode.fragments.join("\n");
                    let _ = writeln!(file_create, "fragments:\n{}", fragments_str);
        },
    }
}
