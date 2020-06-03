use argonautica::{Error as ArgonError, Hasher, Verifier};
use std::error::Error;
use std::fmt;

pub type EncryptionResult<T> = Result<T, EncryptionError>;

#[derive(Debug)]
pub enum EncryptionError {
    HashError(ArgonError),
    VerifyError(ArgonError),
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EncryptionError::*;

        match self {
            HashError(s) => write!(f, "Error hashing password: {}", s),
            VerifyError(s) => write!(f, "Error verifying password: {}", s),
        }
    }
}

impl Error for EncryptionError {
    /// Argonautica errors do not impl `std::error::Error`, so just return None
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn hash(secret: &str, password: &str) -> EncryptionResult<String> {
    Ok(Hasher::default()
        .with_secret_key(secret)
        .with_password(password)
        .hash()
        .map_err(EncryptionError::HashError)?)
}

pub fn verify(secret: &str, hash: &str, password: &str) -> EncryptionResult<bool> {
    Ok(Verifier::default()
        .with_secret_key(secret)
        .with_hash(hash)
        .with_password(password)
        .verify()
        .map_err(EncryptionError::VerifyError)?)
}
