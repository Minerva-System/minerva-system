//! This module wraps functions related to password encryption.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Generate a hash from a slice. Returns the actual bytes of
/// the hashing process. If the hash cannot be generated, panics.
///
/// To have this function work properly, one should call `init_hasher`
/// when initializing whathever module or test this function is used on.
pub fn generate_hash(password: &str) -> Vec<u8> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Cannot generate hash from string")
        .to_string()
        .as_bytes()
        .to_vec()
}

/// Check whether a given password matches a given generated hash.
/// Use this function for password authentication.
pub fn check_hash(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).expect("Cannot parse password hash");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
