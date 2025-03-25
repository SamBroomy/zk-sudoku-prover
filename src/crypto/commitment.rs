use bytes::Bytes;
use rand::TryRngCore;
use std::marker::PhantomData;
use thiserror::Error;

use crate::Value;

#[derive(Debug, Clone, Copy)]
pub struct Hidden;
#[derive(Debug, Clone, Copy)]
pub struct Revealed;

#[derive(Debug, Clone)]
pub struct CommitmentKey {
    value: Value,
    nonce: Bytes,
}

impl CommitmentKey {
    pub fn value(&self) -> Value {
        self.value
    }

    pub fn nonce(&self) -> &[u8] {
        &self.nonce
    }

    pub(crate) fn new(value: Value, nonce: Bytes) -> Self {
        Self { value, nonce }
    }
}

#[derive(Debug, Clone)]
pub struct Commitment<S = Hidden> {
    // Common fields
    hash: Bytes,    // The committed hash
    node_id: usize, // The node this commitment is for
    // State-specific fields
    key: Option<CommitmentKey>,
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
                key: None,
                _marker: PhantomData,
            },
            CommitmentKey { value, nonce },
        )
    }

    /// Reveal the commitment with a key
    /// Can only get a Commitment<Revealed> if the key is correct
    pub fn reveal(self, key: CommitmentKey) -> Result<Commitment<Revealed>, CommitmentError> {
        match self.verify_hash(&key) {
            false => Err(CommitmentError::InvalidReveal),
            true => Ok(Commitment {
                hash: self.hash,
                node_id: self.node_id,
                key: Some(key),
                _marker: PhantomData,
            }),
        }
    }
}

impl Commitment<Revealed> {
    /// Get the revealed value
    pub fn key(&self) -> &CommitmentKey {
        // SAFETY: This is safe because we are in the Revealed state
        // and the key is guaranteed to be present.
        unsafe { self.key.as_ref().unwrap_unchecked() }
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
fn generate_nonce(length: usize) -> Bytes {
    let mut nonce = vec![0u8; length];
    rand::rng().try_fill_bytes(&mut nonce).unwrap();
    Bytes::from_owner(nonce)
}

/// Compute a hash for a value and nonce
fn compute_hash(value: Value, nonce: &[u8]) -> Bytes {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&[value.to_numeric()]);
    hasher.update(nonce);
    Bytes::copy_from_slice(hasher.finalize().as_bytes())
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
        let revealed_key = revealed.key().clone();
        assert_eq!(revealed_key.value, Value::Five);
        assert_eq!(revealed_key.nonce, key.nonce);
    }

    #[test]
    fn test_invalid_reveal() {
        let (commitment, _) = Commitment::new(Value::Five, 1);
        let invalid_key = CommitmentKey {
            value: Value::Six,
            nonce: Bytes::from(vec![0; 32]),
        };
        assert!(commitment.reveal(invalid_key).is_err());
    }

    #[test]
    fn test_commitment_creation_and_revelation() {
        let node_id = 42;
        let value = Value::Three;

        // Create a commitment
        let (commitment, key) = Commitment::new(value, node_id);

        // Verify the commitment properties
        assert_eq!(commitment.node_id(), node_id);
        assert!(!commitment.hash().is_empty());

        // Reveal the commitment
        let revealed = commitment.clone().reveal(key.clone()).unwrap();

        // Verify the revealed commitment
        assert_eq!(revealed.node_id(), node_id);
        assert_eq!(revealed.hash(), commitment.hash());
        assert_eq!(revealed.key().value(), value);
        assert_eq!(revealed.key().nonce(), key.nonce());
    }

    #[test]
    fn test_invalid_reveals() {
        let (commitment, _) = Commitment::new(Value::Five, 1);

        // Test with wrong value
        let invalid_value_key = CommitmentKey {
            value: Value::Six,
            nonce: vec![0; 32].into(),
        };
        assert!(commitment.clone().reveal(invalid_value_key).is_err());

        // Test with wrong nonce
        let invalid_nonce_key = CommitmentKey {
            value: Value::Five,
            nonce: vec![1; 32].into(),
        };
        assert!(commitment.reveal(invalid_nonce_key).is_err());
    }

    #[test]
    fn test_multiple_commitments() {
        // Create multiple commitments
        let (commitment1, key1) = Commitment::new(Value::One, 1);
        let (commitment2, key2) = Commitment::new(Value::Two, 2);
        let (commitment3, key3) = Commitment::new(Value::Three, 3);

        // Reveal in different order
        let revealed2 = commitment2.reveal(key2).unwrap();
        let revealed1 = commitment1.reveal(key1).unwrap();
        let revealed3 = commitment3.reveal(key3).unwrap();

        // Verify the revealed values
        assert_eq!(revealed1.key().value(), Value::One);
        assert_eq!(revealed2.key().value(), Value::Two);
        assert_eq!(revealed3.key().value(), Value::Three);

        // Verify node IDs maintained
        assert_eq!(revealed1.node_id(), 1);
        assert_eq!(revealed2.node_id(), 2);
        assert_eq!(revealed3.node_id(), 3);
    }

    #[test]
    fn test_same_value_different_commitments() {
        // Two commitments with the same value should have different hashes
        let (commitment1, _) = Commitment::new(Value::Seven, 5);
        let (commitment2, _) = Commitment::new(Value::Seven, 5);

        assert_ne!(commitment1.hash(), commitment2.hash());
    }

    #[test]
    fn test_cloning_behavior() {
        // Test that cloning works correctly
        let (commitment, key) = Commitment::new(Value::Four, 10);
        let cloned_commitment = commitment.clone();

        // Original should still work
        let revealed = commitment.reveal(key.clone()).unwrap();
        assert_eq!(revealed.key().value(), Value::Four);

        // Clone should also work
        let revealed_clone = cloned_commitment.reveal(key).unwrap();
        assert_eq!(revealed_clone.key().value(), Value::Four);
    }

    #[test]
    fn test_hash_verification() {
        let value = Value::Nine;
        let nonce: Bytes = vec![1, 2, 3, 4, 5].into();
        let hash = compute_hash(value, &nonce);

        // Create a commitment with same parameters
        let commitment = Commitment::<Hidden> {
            hash: hash.clone(),
            node_id: 99,
            key: None,
            _marker: PhantomData,
        };

        // Verify hash checking works
        let key = CommitmentKey {
            value,
            nonce: nonce.clone(),
        };
        assert!(commitment.verify_hash(&key));

        // Verify wrong value fails
        let wrong_key = CommitmentKey {
            value: Value::One,
            nonce: nonce.clone(),
        };
        assert!(!commitment.verify_hash(&wrong_key));

        // Verify wrong nonce fails
        let wrong_nonce_key = CommitmentKey {
            value,
            nonce: vec![9, 9, 9].into(),
        };
        assert!(!commitment.verify_hash(&wrong_nonce_key));
    }

    #[test]
    fn test_compute_hash_consistency() {
        let value = Value::Six;
        let nonce = vec![7, 8, 9, 10];

        // Computing the same hash twice should yield the same result
        let hash1 = compute_hash(value, &nonce);
        let hash2 = compute_hash(value, &nonce);
        assert_eq!(hash1, hash2);
    }
}
