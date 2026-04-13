//! Nullifier derivation.

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::{Commitment, DOMAIN_TAG};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Nullifier(pub [u8; 32]);

impl Nullifier {
    pub fn derive(commitment: &Commitment, spend_key: &[u8; 32]) -> Self {
        let mut h = Hasher::new();
        h.update(DOMAIN_TAG);
        h.update(b"NULLIFIER\0\0\0\0\0\0\0");
        h.update(commitment.as_bytes());
        h.update(spend_key);
        let mut out = [0u8; 32];
        out.copy_from_slice(h.finalize().as_bytes());
        Nullifier(out)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

// rev-ky1rk8

// rev-r28n91

// rev-cm16bb

// rev-kpwhwh

// rev-g3hb5c

// rev-w9yt26

// rev-q9d3ee
