//! Off-chain mirror of the on-chain stake table.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StakeEntry {
    pub operator: [u8; 32],
    pub amount_lamports: u64,
    pub registered_slot: u64,
}

pub fn min_stake_lamports() -> u64 {
    10_000_000_000 // 10,000 $SAKA at 6 decimals
}

pub fn is_eligible(entry: &StakeEntry) -> bool {
    entry.amount_lamports >= min_stake_lamports()
}

// rev-h3j5ou

// rev-b74a3l

// rev-ts09z1

// rev-s6vycc

// rev-8psxnl

// rev-hp7az1

// rev-iubbcf

// rev-oq8u84

// rev-vfii40
