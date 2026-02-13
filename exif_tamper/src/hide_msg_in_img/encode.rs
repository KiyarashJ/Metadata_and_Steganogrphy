use anyhide::crypto::KeyPair;
use anyhide::encode;
use crate::hide_msg_in_img::{read_file_name::read_file, write_on_file::writing};
use crate::hide_msg_in_img::write_on_file::Res;

pub struct EncodeResult {
    pub code: String,
    pub public_key: x25519_dalek::PublicKey,
    pub secret_key: x25519_dalek::StaticSecret,
}

pub fn encode_img(cr: &std::path::PathBuf , msg: &str, passphrase: &str) -> &'static str {
    let carrier = read_file(cr.to_owned());
    let pair_keys = KeyPair::generate();
    let public_key = pair_keys.public_key();
    let encoding = encode(&carrier, msg, passphrase, public_key).unwrap();
    let data = EncodeResult {
        code: encoding.code,
        public_key: public_key.to_owned(),
        secret_key: pair_keys.secret_key().to_owned(),
    };
    let _ = writing(Res::Encode(data), "Secrets", "txt");
    "done"
}

