use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher as ArgonTrait, PasswordVerifier, SaltString,
    },
    Argon2,
};
use domain::traits::password_hasher::{HashingError, PasswordHasher};
use tracing::{debug, error};

#[derive(Default)]
pub struct Argon2Hasher;

impl Argon2Hasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2Hasher {
    fn hash_password(&self, password: &str) -> Result<String, HashingError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| HashingError::InvalidHash(e.to_string()))?;

        Ok(password_hash.to_string())
    }

    fn verify_password(&self, password: &str, stored_hash: &str) -> Result<bool, HashingError> {
        debug!(
            "Attempting to verify password against hash: {}",
            stored_hash
        );
        let parsed_hash = PasswordHash::new(stored_hash).map_err(|e| {
            error!("Failed to parse hash: {}", e);
            HashingError::InvalidHash(e.to_string())
        })?;

        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => {
                debug!("Password verification successful");
                Ok(true)
            }
            Err(argon2::password_hash::Error::Password) => {
                debug!("Password verification failed: Invalid password");
                Ok(false)
            }
            Err(e) => {
                error!("Password verification error: {}", e);
                Err(HashingError::InvalidHash(e.to_string()))
            }
        }
    }
}
