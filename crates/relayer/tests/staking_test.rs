use sakasu_relayer::staking::{is_eligible, min_stake_lamports, StakeEntry};

#[test]
fn min_stake_is_10k() {
    assert_eq!(min_stake_lamports(), 10_000_000_000);
}

#[test]
fn under_min_is_not_eligible() {
    let e = StakeEntry {
        operator: [0u8; 32],
        amount_lamports: 9_999_999_999,
        registered_slot: 1,
    };
    assert!(!is_eligible(&e));
}

#[test]
fn at_or_above_min_is_eligible() {
    let e = StakeEntry {
        operator: [0u8; 32],
        amount_lamports: 10_000_000_000,
        registered_slot: 1,
    };
    assert!(is_eligible(&e));
}
