//! # Encryption Service
//!
//! Centralizes all cryptographic operations so the rest of the
//! codebase never touches raw hashing primitives directly.
//!
//! ## Algorithm: argon2id
//! Argon2id is the recommended password hashing algorithm (OWASP,
//! RFC 9106). It's resistant to both GPU and side-channel attacks.
//! The `argon2` crate uses safe defaults:
//! - Memory cost:  19 MiB
//! - Time cost:    2 iterations
//! - Parallelism:  1 lane
//! - Output:       32 bytes
//!
//! ## Requirement 16 — Security
//! Passwords are never stored in plaintext. The hash output includes
//! the algorithm parameters and a random salt in PHC string format,
//! e.g. `$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>`.
//! This means the salt doesn't need a separate column.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::errors::{AppError, Result};

/// Hashes a plaintext password using argon2id with a random salt.
///
/// Returns the full PHC-format string (includes algorithm, params,
/// salt, and hash) ready to be stored in the `password_hash` column.
///
/// # Errors
/// Returns `AppError::Internal` if the hashing operation fails,
/// which should only happen on extreme resource constraints.
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {e}")))?;

    Ok(hash.to_string())
}

/// Verifies a plaintext password against a stored argon2id hash.
///
/// The stored hash is in PHC format, so the function extracts the
/// algorithm, parameters, and salt automatically. This means hash
/// upgrades (e.g. increasing memory cost) are backward-compatible:
/// old hashes still verify correctly.
///
/// # Returns
/// - `Ok(true)`  — password matches the hash.
/// - `Ok(false)` — password does not match.
/// - `Err(...)` — the stored hash string is malformed.
pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|e| AppError::Internal(format!("Malformed password hash: {e}")))?;

    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_roundtrip() {
        let password = "hunter2_but_stronger!";
        let hash = hash_password(password).expect("hashing should succeed");

        // Correct password verifies.
        assert!(verify_password(password, &hash).unwrap());

        // Wrong password does not verify.
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn different_hashes_for_same_password() {
        // Each call generates a unique salt, so hashes must differ.
        let h1 = hash_password("same_password").unwrap();
        let h2 = hash_password("same_password").unwrap();
        assert_ne!(h1, h2);
    }
}