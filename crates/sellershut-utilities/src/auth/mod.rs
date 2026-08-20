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

#[cfg(test)]
mod tests {
    use crate::auth::{hash_token, random_token};

    #[track_caller]
    fn check_hash(token: &str, second: &str, expected_result: bool) {
        let actual_result = hash_token(token);
        let second_hash = hash_token(second);
        assert_eq!(actual_result.eq(&second_hash), expected_result);
    }

    #[track_caller]
    fn check_token(expected_result: bool) {
        let actual_result = random_token();
        let second_hash = random_token();
        assert_eq!(actual_result.ne(&second_hash), expected_result);
    }

    #[test]
    fn hash_samesies() {
        let value = "we testing";
        check_hash(value, value, true);
        let second = "other";
        check_hash(value, second, false);
    }

    #[test]
    fn token_gen() {
        check_token(true);
    }
}
