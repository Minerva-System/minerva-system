//! This module wraps functions related to password encryption.

use sodiumoxide::crypto::pwhash::argon2id13;

/// Initialize password hasher.
pub fn init_hasher() {
    sodiumoxide::init().unwrap();
}

/// Generate a hash from a slice. Returns the actual bytes of
/// the hashing process. If the hash cannot be generated, panics.
///
/// To have this function work properly, one should call `init_hasher`
/// when initializing whathever module or test this function is used on.
pub fn generate_hash(password: &str) -> Vec<u8> {
    argon2id13::pwhash(
        password.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .expect("Cannot generate hash from string")
    .0
    .to_vec()
}

/// Check whether a given password matches a given generated hash.
/// Use this function for password authentication.
pub fn check_hash(password: &str, hash: &[u8]) -> bool {
    match argon2id13::HashedPassword::from_slice(hash) {
        Some(pwhash) => argon2id13::pwhash_verify(&pwhash, password.as_bytes()),
        None => false,
    }
}
