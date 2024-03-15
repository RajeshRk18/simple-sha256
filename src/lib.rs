#![no_std]

extern crate alloc;

mod traits;
mod hasher;
mod utils;

pub use hasher::Sha256Hasher;
pub use traits::Hasher;
#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use super::*;
    use self::{hasher::Sha256Hasher, traits::Hasher};

    #[test]
    fn test0() {
        let act = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string();
        let mut hasher = Sha256Hasher::new();
        hasher.update("hello world".as_bytes());
        let hash = hasher.finish();
        assert_eq!(act, hex::encode(hash.0));
    }

    #[test]
    fn test1() {
        let exp: [u8;32] = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14,
            0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
            0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c,
            0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55
        ];
        let mut hasher = Sha256Hasher::new();
        hasher.update(&[]);
        let fin = hasher.finish();
        assert_eq!(exp, fin.0);
    }
    
    #[test]
    fn test2() {
        let exp: [u8;32] = [
            0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08,
            0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d, 0xab, 0xfa,
            0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee,
            0x90, 0x88, 0xf7, 0xac, 0xe2, 0xef, 0xcd, 0xe9
        ];
        let mut hasher = Sha256Hasher::new();
        hasher.update("hello world".as_bytes());
        let fin = hasher.finish();
        assert_eq!(exp,fin.0);
    }
}
