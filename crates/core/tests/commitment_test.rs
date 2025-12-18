use sakasu_core::Commitment;

#[test]
fn commitment_is_deterministic() {
    let vk = [7u8; 32];
    let a = Commitment::build(101, "USDC", 250_000_000, &vk).unwrap();
    let b = Commitment::build(101, "USDC", 250_000_000, &vk).unwrap();
    assert_eq!(a, b);
}

#[test]
fn rejects_empty_asset() {
    let vk = [0u8; 32];
    assert!(Commitment::build(1, "", 100, &vk).is_err());
}

#[test]
fn rejects_oversize_asset() {
    let vk = [0u8; 32];
    assert!(Commitment::build(1, "thisIsTooLongForAsset", 100, &vk).is_err());
}

#[test]
fn different_view_keys_yield_different_commitments() {
    let a = Commitment::build(1, "SOL", 10, &[1u8; 32]).unwrap();
    let b = Commitment::build(1, "SOL", 10, &[2u8; 32]).unwrap();
    assert_ne!(a, b);
}
