//! Cross-chain shielded transfer envelope.

use serde::{Deserialize, Serialize};

use crate::{Commitment, Nullifier};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transfer {
    pub from_chain: u16,
    pub to_chain: u16,
    pub asset: String,
    pub amount: u128,
    pub commitment: Commitment,
    pub nullifier: Nullifier,
    pub deadline_slot: u64,
}

impl Transfer {
    pub fn id(&self) -> [u8; 32] {
        *self.commitment.as_bytes()
    }
}

// rev-z6tzxc

// rev-rbauxu

// rev-f3tgnr

// rev-zobgd4

// rev-1dt9cu
