# Sakasu ZK circuits

This directory holds the Groth16 circuit that the on-chain Solana verifier
checks before accepting a `commit_transfer` call.

## What the circuit proves

```
public  commitment_hash
private chain_id, asset_id, amount, view_key

statement: commitment_hash == Poseidon(chain_id, asset_id, amount, view_key)
```

The prover (a Sakasu user, locally in their browser or via the relayer SDK)
demonstrates they know the four witnesses that hash to a published
`commitment_hash`. The on-chain verifier sees only the proof bytes and the
public output; the four witnesses never leave the prover.

Curve: **BN-254** (alt_bn128) — Solana provides `alt_bn128_pairing` as a
native syscall, so the verifier runs on-chain in well under the 200K
compute-unit ceiling.

## Files

- `circuits/commitment_proof.circom` — the circuit, 736 constraints.
- `build/commitment_proof.r1cs` — compiled R1CS constraint system.
- `build/commitment_proof_final.zkey` — Groth16 proving key (Phase 2
  finalised). Derived from the iden3 community Powers of Tau ceremony,
  `powersOfTau28_hez_final_12.ptau`.
- `build/verification_key.json` — Groth16 verification key. Matches the
  `vk_*` constants embedded in `programs/sakasu-vault/src/verifier.rs`.
- `build/commitment_proof_js/` — generated WASM witness builder used by
  off-chain provers and by the TypeScript SDK.
- `test_commitment_proof.js` — end-to-end smoke test:
  1. compute reference Poseidon hash with circomlibjs;
  2. generate a Groth16 proof via snarkjs;
  3. verify against the verification key;
  4. tamper test (change `commitment_hash` by +1, must reject).

## Running the test

```
cd zk
npm install
node test_commitment_proof.js
```

Expected: `=== ALL CHECKS PASSED ===` and a proof generation time well
under one second on commodity hardware.

## Live mainnet verifier (v1.0, 2026-05-18)

The verification key in `build/verification_key.json` is the same key
the on-chain Solana program at
`HXXFgjuzwhNzk4EfGf6pWNE5hapV5FYN2ZRSwchcMy8p` uses when it runs the
BN-254 pairing check via the `alt_bn128_pairing` syscall.

First production `commit_transfer` carrying a real Groth16 proof:
[3KjW7Jyr...ddK2](https://solscan.io/tx/3KjW7JyrzE79GJi8LhXxisuCsfggaXJb7EzY2gUQHBwBmMTzadW8uMnxtttsjSyS5Y6sYMmpBEmKdAiVuT55ddK2).

The transaction logs show the program invoking the BN-254 syscalls — the
proof bytes never decode into anything readable on-chain; the verifier
sees only the curve points and runs the pairing equation. Anyone with
`snarkjs` can produce a fresh proof against this verification key and
get it accepted by the same program.
