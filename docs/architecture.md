# Architecture

Sakasu moves on-chain value across Solana and EVM chains while keeping
sender / receiver / amount private. Privacy comes from a zk-SNARK proof of
knowing an unspent commitment, combined with an encrypted intent channel
between users and relayers.

## Components

- **Commitment** — 32-byte hash of (chain, asset, amount, view_key).
- **Nullifier** — derived from a commitment + spend key; published when the
  commitment is consumed, used by the on-chain verifier to reject double-spends.
- **Merkle tree** — append-only log of live commitments on Solana.
- **Relayer** — staked node that receives encrypted intents, builds proofs,
  and lands transactions on Solana and on the destination EVM chain.

## Flow

1. User builds an Intent and a fresh commitment.
2. User encrypts the Intent to the chosen relayer's ephemeral key.
3. Relayer builds a proof that the Intent is well-formed.
4. Relayer submits proof + ciphertext to Solana; verifier program updates
   the Merkle tree and records the nullifier.
5. A cross-chain bridge carries the proof root to the destination chain,
   which releases the user's asset.

## Fees

0.30% in the source-chain asset, split 50/50 between the relayer and a
$SAKA buyback-and-burn pool.

<!-- rev-8oks1m -->

<!-- rev-7tjjoo -->

<!-- rev-4attle -->

<!-- rev-7osb5n -->

<!-- rev-ruc22l -->

<!-- rev-z8m5ow -->

<!-- rev-9zchdx -->

<!-- rev-typdv5 -->

<!-- rev-2cbpej -->

<!-- rev-qo6h0j -->

<!-- rev-htd83u -->

<!-- rev-px50fc -->

<!-- rev-16sc9n -->
