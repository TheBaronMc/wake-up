use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

use crate::configuration::CONFIGURATION;




pub fn create_token() -> Result<String, String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(get_pass().as_str().as_bytes()).expect("The key length must be greater than zero");
    let mut claims = BTreeMap::new();
    claims.insert("Auth", true);

    match claims.sign_with_key(&key) {
        Err(error) => Err(error.to_string()),
        Ok(token)  => Ok(token)
    }
}

pub fn verify_token(token: &str) -> bool {
    let key: Hmac<Sha256> = Hmac::new_from_slice(get_pass().as_str().as_bytes()).expect("The key length must be greater than zero");
    let result: Result<BTreeMap<String, String>, jwt::Error> = token.verify_with_key(&key);
    match result {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn verify_pass(password: &str) -> bool {
    get_pass().as_str() == password
}

fn get_pass() -> String {
    String::from(CONFIGURATION.read().expect("Unable to read configuration").password.as_str())
}