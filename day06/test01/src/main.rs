//use rand;
use base64::prelude::*;
use rand;
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use std::fs;
fn main() {
    let msg: &str = "Hello, World!";
    let public_key_pem = fs::read_to_string("D:/aprepare/hello-wasm/public_key.pem").unwrap();
    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem).unwrap();
    let mut rng = rand::thread_rng();
    let enc_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes())
        .unwrap();
    let base64_enc_data = BASE64_STANDARD.encode(enc_data);
    println!("Encrypted data: {:?}", base64_enc_data);

    let private_key_pem = fs::read_to_string("D:/aprepare/hello-wasm/private_key.pem").unwrap();
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem).unwrap();
    let data = BASE64_STANDARD.decode(base64_enc_data).unwrap();
    let plain_data = private_key.decrypt(Pkcs1v15Encrypt, &data[..]).unwrap();
    let result = String::from_utf8(plain_data).unwrap();
    println!("Decrypted data: {:?}", result);
}
