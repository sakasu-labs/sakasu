use sakasu_core::{Commitment, Nullifier};

#[test]
fn nullifier_is_deterministic() {
    let vk = [3u8; 32];
    let sk = [4u8; 32];
    let c = Commitment::build(101, "USDC", 1, &vk).unwrap();
    let a = Nullifier::derive(&c, &sk);
    let b = Nullifier::derive(&c, &sk);
    assert_eq!(a, b);
}

#[test]
fn different_spend_keys_yield_different_nullifiers() {
    let vk = [3u8; 32];
    let c = Commitment::build(101, "USDC", 1, &vk).unwrap();
    let a = Nullifier::derive(&c, &[1u8; 32]);
    let b = Nullifier::derive(&c, &[2u8; 32]);
    assert_ne!(a, b);
}
