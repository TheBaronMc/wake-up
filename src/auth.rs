use hmac::{Hmac, Mac};
use jwt::{Claims, RegisteredClaims, SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::configuration::read_configuration;

pub fn create_token() -> Result<String, String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(get_pass().as_str().as_bytes())
        .expect("The key length must be greater than zero");

    let since_epoch_timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let claims = Claims::new(RegisteredClaims {
        expiration: Some(3600 + since_epoch_timestamp),
        ..Default::default()
    });

    match claims.sign_with_key(&key) {
        Err(error) => Err(error.to_string()),
        Ok(token) => Ok(token),
    }
}

pub fn verify_token(token: &str) -> bool {
    let key: Hmac<Sha256> = Hmac::new_from_slice(get_pass().as_str().as_bytes())
        .expect("The key length must be greater than zero");
    let result: Result<RegisteredClaims, jwt::Error> = token.verify_with_key(&key);

    if let Err(error) = result {
        debug!("{error}");
        false
    } else {
        true
    }
}

pub fn verify_pass(password: &str) -> bool {
    get_pass().as_str() == password
}

fn get_pass() -> String {
    read_configuration().unwrap().password().clone()
}
