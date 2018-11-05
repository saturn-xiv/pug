#[cfg(feature = "sodium")]
pub mod sodium;

use serde::{de::DeserializeOwned, ser::Serialize};

use super::errors::Result;

pub trait Encryptor {
    fn encrypt<V: Serialize>(&self, plain: &V) -> Result<(Vec<u8>, Vec<u8>)>;
    fn decrypt<V: DeserializeOwned>(&self, cipher: &[u8], nonce: &[u8]) -> Result<V>;

    fn sum(plain: &[u8]) -> Result<Vec<u8>>;
    fn verify(cipher: &[u8], plain: &[u8]) -> bool;

    fn random(l: usize) -> Vec<u8>;
}
