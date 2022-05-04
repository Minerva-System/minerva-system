use sodiumoxide::crypto::pwhash::argon2id13;

pub fn init_hasher() {
    sodiumoxide::init().unwrap();
}

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

pub fn check_hash(password: &str, hash: &[u8]) -> bool {
    match argon2id13::HashedPassword::from_slice(hash) {
        Some(pwhash) => argon2id13::pwhash_verify(&pwhash, password.as_bytes()),
        None => false,
    }
}
