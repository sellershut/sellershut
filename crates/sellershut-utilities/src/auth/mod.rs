use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::{TryRng, rngs::SysRng};
use sha2::{Digest, Sha256};

pub fn hash_token(token: &str) -> Vec<u8> {
    Sha256::digest(token.as_bytes()).to_vec()
}

pub fn random_token() -> String {
    let mut bytes = [0_u8; 32];
    let mut rng = SysRng;
    rng.try_fill_bytes(&mut bytes).unwrap();
    URL_SAFE_NO_PAD.encode(bytes)
}
