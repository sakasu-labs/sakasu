//! Minimal append-only Merkle tree used by the on-chain verifier mock.
//! Real implementation uses a poseidon-style hash; this is blake3 for tests.

use blake3::Hasher;

use crate::DOMAIN_TAG;

#[derive(Clone, Debug, Default)]
pub struct MerkleTree {
    leaves: Vec<[u8; 32]>,
}

impl MerkleTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, leaf: [u8; 32]) -> usize {
        self.leaves.push(leaf);
        self.leaves.len() - 1
    }

    pub fn len(&self) -> usize {
        self.leaves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }

    /// Compute the current root.
    pub fn root(&self) -> [u8; 32] {
        if self.leaves.is_empty() {
            return [0u8; 32];
        }
        let mut layer: Vec<[u8; 32]> = self.leaves.clone();
        while layer.len() > 1 {
            let mut next = Vec::with_capacity(layer.len().div_ceil(2));
            for chunk in layer.chunks(2) {
                let mut h = Hasher::new();
                h.update(DOMAIN_TAG);
                h.update(b"MERKLE_NODE\0\0\0\0\0");
                h.update(&chunk[0]);
                if chunk.len() == 2 {
                    h.update(&chunk[1]);
                } else {
                    h.update(&chunk[0]); // duplicate odd leaf
                }
                let mut out = [0u8; 32];
                out.copy_from_slice(h.finalize().as_bytes());
                next.push(out);
            }
            layer = next;
        }
        layer[0]
    }
}

// rev-ysydmq

// rev-ic1onr

// rev-cqin3y

// rev-0l79ze

// rev-3gsfk8

// rev-vjhrvj

// rev-nxgu2c

// rev-485ape

// rev-crlbs0

// rev-3q8iw1

// rev-xk6lo8
