use rand::{TryRngCore, rngs::OsRng};
use sha2::{Digest, Sha256};
use std::marker::PhantomData;
use thiserror::Error;

use crate::Value;

#[derive(Debug, Clone, Copy)]
pub struct Hidden;
#[derive(Debug, Clone, Copy)]
pub struct Revealed;

trait CommitmentState {}
impl CommitmentState for Hidden {}
impl CommitmentState for Revealed {}

#[derive(Debug, Clone)]
pub struct CommitmentKey {
    value: Value,
    nonce: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Commitment<S = Hidden> {
    // Common fields
    hash: Vec<u8>,  // The committed hash
    node_id: usize, // The node this commitment is for
    // State-specific fields
    value: Option<Value>,
    nonce: Option<Vec<u8>>,
    _marker: PhantomData<S>,
}

impl Commitment<Hidden> {
    /// Create a new commitment for a value
    pub fn new(value: Value, node_id: usize) -> (Self, CommitmentKey) {
        let nonce = generate_nonce(32); // 32 bytes of randomness
        let hash = compute_hash(value, &nonce);

        (
            Self {
                hash,
                node_id,
                value: None,
                nonce: None,
                _marker: PhantomData,
            },
            CommitmentKey { value, nonce },
        )
    }

    pub fn reveal(self, key: CommitmentKey) -> Result<Commitment<Revealed>, CommitmentError> {
        match self.verify_hash(&key) {
            false => Err(CommitmentError::InvalidReveal),
            true => Ok(Commitment {
                hash: self.hash,
                node_id: self.node_id,
                value: Some(key.value),
                nonce: Some(key.nonce),
                _marker: PhantomData,
            }),
        }
    }
}

impl Commitment<Revealed> {
    /// Get the revealed value
    pub fn value(&self) -> Value {
        self.value.unwrap()
    }

    /// Get the nonce used for the commitment
    pub fn nonce(&self) -> &[u8] {
        self.nonce.as_ref().unwrap()
    }
}

// Common functionality for both states
impl<S> Commitment<S> {
    pub fn node_id(&self) -> usize {
        self.node_id
    }

    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    // Helper for validation
    fn verify_hash(&self, key: &CommitmentKey) -> bool {
        compute_hash(key.value, &key.nonce) == self.hash
    }
}

/// Generate a cryptographically secure random nonce
fn generate_nonce(length: usize) -> Vec<u8> {
    let mut nonce = vec![0u8; length];
    OsRng.try_fill_bytes(&mut nonce).unwrap();
    nonce
}

/// Compute a hash for a value and nonce
fn compute_hash(value: Value, nonce: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update([value.to_numeric()]);
    hasher.update(nonce);
    hasher.finalize().to_vec()
}

#[derive(Debug, Error)]
pub enum CommitmentError {
    #[error("Invalid reveal - hash does not match")]
    InvalidReveal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment() {
        let (commitment, key) = Commitment::new(Value::Five, 1);
        let revealed = commitment.reveal(key.clone()).unwrap();
        assert_eq!(revealed.value(), Value::Five);
        assert_eq!(revealed.nonce(), &key.nonce);
    }

    #[test]
    fn test_invalid_reveal() {
        let (commitment, _) = Commitment::new(Value::Five, 1);
        let invalid_key = CommitmentKey {
            value: Value::Six,
            nonce: vec![0; 32],
        };
        assert!(commitment.reveal(invalid_key).is_err());
    }
}
