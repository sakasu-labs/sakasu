//! Shielded commitments.

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::DOMAIN_TAG;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Commitment(pub [u8; 32]);

impl Commitment {
    pub fn build(
        chain_id: u16,
        asset: &str,
        amount: u128,
        view_key: &[u8; 32],
    ) -> Result<Self, Error> {
        if asset.is_empty() || asset.len() > 16 {
            return Err(Error::InvalidAsset);
        }
        let mut hasher = Hasher::new();
        hasher.update(DOMAIN_TAG);
        hasher.update(&chain_id.to_le_bytes());
        let mut padded = [0u8; 16];
        padded[..asset.len()].copy_from_slice(asset.as_bytes());
        hasher.update(&padded);
        hasher.update(&amount.to_le_bytes());
        hasher.update(view_key);
        let mut out = [0u8; 32];
        out.copy_from_slice(hasher.finalize().as_bytes());
        Ok(Commitment(out))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
