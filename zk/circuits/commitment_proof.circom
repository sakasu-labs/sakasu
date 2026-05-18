pragma circom 2.1.6;

include "../node_modules/circomlib/circuits/poseidon.circom";

// Sakasu Commitment Proof
//
// Proves the prover knows (chain_id, asset_id, amount, view_key) such that
//   commitment_hash = Poseidon(chain_id, asset_id, amount, view_key)
//
// Public  inputs:  commitment_hash
// Private inputs:  chain_id, asset_id, amount, view_key
//
// On-chain verifier (Solana alt_bn128) checks the Groth16 proof against this
// statement before accepting a commit_transfer call. That is the v1 guarantee:
// the on-chain program never sees the four witness values, but it does prove
// they exist and produce the published commitment.
template SakasuCommitmentProof() {
    // Public output (mirrors a public input in Groth16)
    signal output commitment_hash;

    // Private witnesses
    signal input chain_id;
    signal input asset_id;
    signal input amount;
    signal input view_key;

    component hasher = Poseidon(4);
    hasher.inputs[0] <== chain_id;
    hasher.inputs[1] <== asset_id;
    hasher.inputs[2] <== amount;
    hasher.inputs[3] <== view_key;

    commitment_hash <== hasher.out;
}

component main = SakasuCommitmentProof();
