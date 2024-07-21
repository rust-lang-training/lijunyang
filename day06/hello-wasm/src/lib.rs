use wasm_bindgen::prelude::*;

use base64::prelude::*;
use rand::{self, thread_rng};
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
const priv_pem: &'static str = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC+et/+GvE+Hofm
T/GfRsucgdul4tigTYefmcWjZrz0e4ZFLlylKBg+WgB04aQ5yxQYnJ1LDPWlGJax
S3Moe6yQsnRZrd9lM889JRhsUiEuwfUQu9CXl/f2c9j08k3KVqIiLC6aNFAq1QPy
uIFr/LjuzK3VgWidIckaSKrjKJooXpafGoSqyrDy4JhWiLUJdK6tu3hXutGHSWB+
VUTVCd5oe2QLkowrVFaUaWv+4Lu93npdUJd9lBm9X/pWp4TtOeR12ooH6YF1bL1s
hibYJAbOwrbAJvLDR3fqNthpEUBxxu+/corLhLuG02urRp0bbW5JRsmmsugtX8m5
En0DiuFXAgMBAAECggEAD1GFawOSrBq7dieuuE7yt/rvhac5IacqhYjRzqLIBmda
tHyWgce4L2eAiuXMlcNBsfsCYVNA/A5+q/Jz5+kORzK6za2AfjyM6Y9up+Nw/Lgn
c7mO/GoBzIWjiItl+yQ9dL8V3lvfqJop756r9dnEq5u1TSrTlXNun1kBuTP+NcnG
DRvVzEMJOimFU1kTQRgjur+wi1Dhl+y+9I1lf9JpMfwqW7/TBtTls0FIMk92baPr
O2OZa8Z91ztere2RflTj3nYhCM3WE4oegbaFAsUZrYBxKIqFs7TpoTzvIGmBZ0D2
WywG4XO9qHZfXdD+ZJF5+UsvoDHSnlMUky3BFbLXTQKBgQDj54esLymg/4wnR/89
yttzqjLRIazALixQufPlQDmOEQgsQRCZo+KxkShNzLiE5Bboio54YH16rxeTAYnX
uSPdh6cksCHoyuxce52RQwwI5y4oYeYaD9kmvAEewlntYEvKTIbo4QED+sHizABD
fR/+vIAFekVIiyhFF06Lqfk+PQKBgQDV9kL5HmcdvELb7A41cfgHHqpS47RPh62F
cXNRfBhF1K0TOGS3v8Rixu2yjitiV5bPLemMETd5jBMfy57C2OW2VQEG0xYq8oZN
X+UThCL4wEFYFuqLzMeyrpVMM38QAHBPMLKnmZyOa73/q7Kn8Ccu3ON59gFp59Xk
jKtHkWHLIwKBgGgj9VgV2QTroSSV/qlC2BK7dCsiIylSMAfXYP6h4Wrsl3p5ZNeA
Dueq9+rNIIgBHgK2xbF2DgCQn8oaCt3biOsCwLrADU6t42HDTtYA+C/dS7dCPP4t
vcG4aR6gGq2gsiFH5OHnZyplXFGBfKFaPNOPWUSngZ9Lj2Q66TZivjTVAoGBALI2
g0QycZsP4vSBIribbAaqIR4XcWlIzlw2yeGWJjWL1nkK4ME4JmBL51JxHmu0HvYD
/OJguL8x0AAMvvj2Zic/mDJSAcXV6c1q0Ug3KFXe9M5lkAjPkYof5lTpE62FYAZl
vnjg9sidr0zUe2NsIYbkD0jXggfvRXYvQL7EthrXAoGAQIskltWUFet9JULxw63e
WiMgH5UzevRUv2GVajYq7ZOlRMzzvukhz0qrcHXjHJA2TfvHE/BCdGz5+f61Q5rw
qvbBv36Z5xyiEpFoltKwjaIC87KZyFBpi9uo6p6IwyPf3bZPF0ZzvvmUyImL6TA8
VG+063QDi5o4bWi9K7Tgh/4=
-----END PRIVATE KEY-----
";
const pub_pem: &'static str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvnrf/hrxPh6H5k/xn0bL
nIHbpeLYoE2Hn5nFo2a89HuGRS5cpSgYPloAdOGkOcsUGJydSwz1pRiWsUtzKHus
kLJ0Wa3fZTPPPSUYbFIhLsH1ELvQl5f39nPY9PJNylaiIiwumjRQKtUD8riBa/y4
7syt1YFonSHJGkiq4yiaKF6WnxqEqsqw8uCYVoi1CXSurbt4V7rRh0lgflVE1Qne
aHtkC5KMK1RWlGlr/uC7vd56XVCXfZQZvV/6VqeE7TnkddqKB+mBdWy9bIYm2CQG
zsK2wCbyw0d36jbYaRFAccbvv3KKy4S7htNrq0adG21uSUbJprLoLV/JuRJ9A4rh
VwIDAQAB
-----END PUBLIC KEY-----";

#[wasm_bindgen]
pub fn encrypt(data: &str) -> String {
    let pub_key = RsaPublicKey::from_public_key_pem(pub_pem).expect("failed to parse public key");
    let bits = 1024;
    let mut rng = thread_rng();
    let data = data.as_bytes();
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");

    BASE64_STANDARD.encode(enc_data)
}
#[wasm_bindgen]
pub fn decrypt(data: &str) -> String {
    let data = BASE64_STANDARD.decode(data).unwrap();
    let priv_key = RsaPrivateKey::from_pkcs8_pem(&priv_pem).expect("failed to generate a key");
    let dec_data = priv_key
        .decrypt(Pkcs1v15Encrypt, &data[..])
        .expect("failed to decrypt");
    String::from_utf8(dec_data).unwrap()
}
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let val = document.create_element("p")?;
    val.set_text_content(Some("hello from rust!"));
    body.append_child(&val)?;
    Ok(())
}

#[wasm_bindgen]
pub fn green(name: &str) -> String {
    format!("hello, {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
