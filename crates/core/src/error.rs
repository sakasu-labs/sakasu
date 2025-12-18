use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("asset symbol must be 1-16 bytes ASCII")]
    InvalidAsset,
    #[error("amount overflow")]
    AmountOverflow,
    #[error("proof verification failed")]
    InvalidProof,
    #[error("merkle path mismatch")]
    MerklePathMismatch,
}
