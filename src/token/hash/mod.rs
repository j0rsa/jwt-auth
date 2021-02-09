use sha2::{Digest, Sha256, Sha512};
use std::str;

pub fn sha256hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(input);
    let result = hasher.result();
    format!("{:x}", result)
}

pub fn sha512hash(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.input(input);
    let result = hasher.result();
    format!("{:x}", result)
}

pub fn verify_bcrypt_hash(password: &str, password_hash: &str) -> bool {
    let result = bcrypt::verify(password, password_hash);
    return result.is_ok() && result.ok().unwrap_or(false);
}

#[cfg(test)]
mod tests {
    use crate::token::hash::{sha256hash, sha512hash, verify_bcrypt_hash};

    #[test]
    fn test_sha256() {
        let hash = sha256hash("test");
        assert_eq!(
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
            hash
        );
    }
    #[test]
    fn test_sha512() {
        let hash = sha512hash("test");
        assert_eq!("ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff", hash);
    }

    #[test]
    fn test_bcrypt_valid() {
        let password = "test123";
        let hash = "$2y$12$Pa/biT5ibBUJuDwXg6hr4.GLedsulQlHMJEA7O/.aXNkm.FF8OysG";
        assert!(verify_bcrypt_hash(password, hash))
    }

    #[test]
    fn test_bcrypt_invalid() {
        let password = "test123";
        let hash = "$2y$12$ulN.fejQw49xAXgFR1YwheYZsCPLxAQIqxCJYgScKeno36bEnqjUq";
        assert_eq!(verify_bcrypt_hash(password, hash), false)
    }
}
