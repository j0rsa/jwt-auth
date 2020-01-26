use sha2::{Digest, Sha256, Sha512};
use std::str;

pub fn sha256hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(input);
    let result = hasher.result();
    format!("{:x}",result)
}

pub fn sha512hash(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.input(input);
    let result = hasher.result();
    format!("{:x}",result)
}

#[cfg(test)]
mod tests {
    use crate::token::sha::{sha256hash, sha512hash};

    #[test]
    fn test_sha256() {
        let hash = sha256hash("test");
        assert_eq!("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08", hash);
    }
    #[test]
    fn test_sha512() {
        let hash = sha512hash("test");
        assert_eq!("ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff", hash);
    }

}