# Circuit notes

The Sakasu circuit proves knowledge of (asset, amount, view_key) such that
`commitment = blake3(domain || chain || asset_padded || amount || view_key)`.

Public inputs: commitment_hash, nullifier_hash, deadline_slot.

The on-chain verifier mirrors the same hash so it can validate the
commitment a relayer claims to be inserting into the Merkle tree.
