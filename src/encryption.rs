use argonautica::{Hasher, Verifier};

pub fn hash(secret: &str, password: &str) -> Result<String, String> {
    Ok(Hasher::default()
        .with_secret_key(secret)
        .with_password(password)
        .hash()
        .map_err(|e| e.to_string())?)
}

pub fn verify(secret: &str, hash: &str, password: &str) -> Result<bool, String> {
    Ok(Verifier::default()
        .with_secret_key(secret)
        .with_hash(hash)
        .with_password(password)
        .verify()
        .map_err(|e| e.to_string())?)
}
