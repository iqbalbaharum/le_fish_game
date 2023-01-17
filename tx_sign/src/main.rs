#![allow(improper_ctypes)]

use std::ops::Deref;

use ed25519_compact::{KeyPair, PublicKey, Signature};
use ed25519_compact::{Noise, SecretKey};
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn sign(message: String, private_key: String) -> String {
    let pk_key_decoded = base64::decode(private_key).unwrap();

    let pk = pk_key_decoded.try_into().expect("invalid private key");

    let sk = SecretKey::new(pk);

    let signature = sk.sign(message, Some(Noise::default()));

    base64::encode(signature)
}
