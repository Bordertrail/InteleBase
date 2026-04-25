//! Password hashing using Argon2id

use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::SaltString,
};

use kb_core::AppError;

/// Argon2id parameters (m=65536, t=3, p=4)
const M_COST: u32 = 65536; // Memory cost
const T_COST: u32 = 3; // Time cost
const P_COST: u32 = 4; // Parallelism

/// Hash a password using Argon2id
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let params = Params::new(M_COST, T_COST, P_COST, None)
        .map_err(|e| AppError::PasswordHashError(e.to_string()))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::PasswordHashError(e.to_string()))?;

    Ok(hash.to_string())
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| AppError::PasswordHashError(e.to_string()))?;

    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    match result {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
