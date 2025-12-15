use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher as ArgonTrait, PasswordVerifier, SaltString,
    },
    Argon2,
};
use domain::{error::domain_error::DomainError, traits::password_hasher::PasswordHasher};

pub struct Argon2Hasher;

impl Argon2Hasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2Hasher {
    fn hash_password(&self, password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| DomainError::PasswordHashingError(e.to_string()))?;

        Ok(password_hash.to_string())
    }

    fn verify_password(&self, password: &str, stored_hash: &str) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(stored_hash)
            .map_err(|e| DomainError::PasswordHashingError(e.to_string()))?;

        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(DomainError::PasswordHashingError(e.to_string())),
        }
    }
}
