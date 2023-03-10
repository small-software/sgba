use std::io::Error;
use argon2::{self, Config};

pub fn generate_from_password(password: &str) -> Result<String, Error> {

    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap();
    let matches = argon2::verify_encoded(&hash, password.as_bytes()).unwrap();
    assert!(matches);

    Ok(hash)
}