//! sakasu-core — shielded transfer primitives.

pub mod commitment;
pub mod error;
pub mod merkle;
pub mod nullifier;
pub mod transfer;

pub use commitment::Commitment;
pub use error::Error;
pub use merkle::MerkleTree;
pub use nullifier::Nullifier;
pub use transfer::Transfer;

pub const DOMAIN_TAG: &[u8; 16] = b"SAKASU_ZK_V0001 ";
pub const PROTOCOL_VERSION: u32 = 2;

// rev-m1vkgi

// rev-5f0xix

// rev-6ei8sy

// rev-r8vwfu

// rev-aate1u

// rev-ykg8jg

// rev-0e8cue
