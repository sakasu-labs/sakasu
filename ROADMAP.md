# Roadmap

This file is the source of truth for what Sakasu ships and when. The README
"Status" section quotes from here.

## v0 — Live on Solana mainnet

Shipped 2026-05-18.

- `sakasu-vault` Anchor program deployed to Solana mainnet at
  `HXXFgjuzwhNzk4EfGf6pWNE5hapV5FYN2ZRSwchcMy8p`.
- Five on-chain instructions:
  - `initialize_vault` — one-time vault bootstrap (called).
  - `commit_transfer` — user side; transfers asset into the vault PDA and
    records a 32-byte commitment.
  - `relayer_redeem` — staked relayer side; releases asset to the address
    encoded in the commitment scheme and burns the nullifier.
  - `register_relayer` — operator side; stakes SOL and creates a
    `RelayerAccount` PDA in `Active` status.
  - `unstake_relayer` — operator side; returns staked principal and marks
    `RelayerAccount` as `Unstaked`.
- Token-2022 compatible vault using `anchor-spl::token_interface`.
- TypeScript SDK with commitment + intent builders (BLAKE3 / SHA-256
  stand-in for the post-Poseidon hash).
- Public statistics aggregator at `api.sakasu.space`.
- Rust relayer crate with a functioning HTTP node skeleton.

## v0.1 — Honest copy + repo hygiene

In flight (this week).

- Strip generator-side revision markers from public source files.
- Tighten README and marketing copy to match what is actually live today
  versus what is on the roadmap. (No "running now" claims for components
  that ship later.)
- Expand `docs/` with explicit "shipped" vs "planned" markers per crate.

## v0.2 — Relayer network real-mode

Target: 1–2 weeks post-launch.

- Replace the relayer HTTP skeleton with a complete listener:
  - mTLS termination on the `/intent` endpoint.
  - Encrypted-intent decryption (`X25519` + `ChaCha20-Poly1305`).
  - Health and metrics endpoints used by the public statistics page.
- Persistent journal for in-flight intents so a relayer can resume after
  restart without losing user funds.
- Multi-node operator-to-operator gossip protocol scaffolding.

## v1.0 — ZK verifier on-chain (shipped 2026-05-18)

Status: **live on Solana mainnet.**

- `commit_transfer` accepts an optional Groth16 proof argument.
- The proof is verified inside the on-chain program using Solana's native
  `alt_bn128_pairing` / `alt_bn128_addition` / `alt_bn128_multiplication`
  syscalls, without any external ark / curve crate.
- Verification key, proving key, and the originating circom circuit are
  committed under `zk/` so anyone can reproduce the math.
- First production `commit_transfer` with a real proof:
  [3KjW7Jyr...ddK2](https://solscan.io/tx/3KjW7JyrzE79GJi8LhXxisuCsfggaXJb7EzY2gUQHBwBmMTzadW8uMnxtttsjSyS5Y6sYMmpBEmKdAiVuT55ddK2).

Still to come before v1.x freezes:
- External security audit of the verifier + the rest of the program.
- After audit closes and a grace window with no critical findings,
  `solana program set-upgrade-authority --final` to freeze the program.

## v1.1 — Cross-chain destination verifiers

Target: alongside v1.0.

- Solidity destination verifiers on Base, Ethereum, and Arbitrum.
- LayerZero (or Wormhole) message routing.
- End-to-end cross-chain shielded transfer demos from the Solana mainnet
  vault to each destination chain.

## v1.2 — Full intent encryption + SDK polish

Target: post-audit.

- `@sakasu/sdk` 0.3: real `X25519 + ChaCha20-Poly1305` envelope encryption.
- Wallet adapter integration for browsers (Phantom / Solflare / Backpack).
- Public docs site for the SDK.

## v2.x — RFQ pool

Target: post-v1.

- Private RFQ filling on top of the same commitment set. Any market maker
  can fill an intent without seeing the address that opened it.

## v3.x — Restaking

Target: post-v2.

- Relayers earn $SKS on top of swap-fee revenue by securing the
  cross-chain message bus.

---

## What each "Target" actually means

The dates above are commitments to ship the work, not commitments to ship
without findings. If an audit comes back with critical findings, the v1.0
target moves out until they are fixed. The repository's commit history is
the audit trail for any slip.
