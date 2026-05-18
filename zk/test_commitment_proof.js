// End-to-end test of the Sakasu commitment proof:
//   1. compute commitment_hash = Poseidon(chain_id, asset_id, amount, view_key)
//      using circomlibjs (Poseidon over BN-254, same curve as the circuit)
//   2. generate a Groth16 proof that we know those four witnesses
//   3. verify the proof against the verification key
//
// Both off-chain (this test) and on-chain (sakasu-vault program upgrade)
// use the exact same verification key bytes.

const path = require('path')
const fs = require('fs')
const snarkjs = require('snarkjs')
const { buildPoseidon } = require('circomlibjs')

const WASM = path.join(__dirname, 'build/commitment_proof_js/commitment_proof.wasm')
const ZKEY = path.join(__dirname, 'build/commitment_proof_final.zkey')
const VKEY = JSON.parse(fs.readFileSync(path.join(__dirname, 'build/verification_key.json'), 'utf-8'))

async function main() {
  console.log('=== Sakasu commitment proof — end-to-end test ===\n')

  const poseidon = await buildPoseidon()
  const F = poseidon.F

  // Witnesses
  const chain_id = 101n // solana mainnet
  const asset_id = BigInt('0x534f4c00000000000000000000000000') // "SOL" padded
  const amount = 10_000_000n // 0.01 SOL in lamports
  const view_key = BigInt('0x' + 'a'.repeat(63) + '1')

  // Step 1 — reference Poseidon hash (matches the circuit's Poseidon)
  const hashElement = poseidon([chain_id, asset_id, amount, view_key])
  const expected = F.toObject(hashElement)
  console.log('reference Poseidon(chain, asset, amount, view_key):')
  console.log('  ' + expected.toString())

  // Step 2 — generate Groth16 proof
  console.log('\ngenerating Groth16 proof...')
  const t0 = Date.now()
  const { proof, publicSignals } = await snarkjs.groth16.fullProve(
    {
      chain_id: chain_id.toString(),
      asset_id: asset_id.toString(),
      amount: amount.toString(),
      view_key: view_key.toString(),
    },
    WASM,
    ZKEY,
  )
  const dt = Date.now() - t0
  console.log(`  proof generated in ${dt}ms`)
  console.log(`  public signals (commitment_hash):`)
  console.log(`    ${publicSignals[0]}`)

  // Check the public output is the same as the reference Poseidon
  if (publicSignals[0] !== expected.toString()) {
    throw new Error('FAIL: circuit output != reference Poseidon')
  }
  console.log('  ✓ circuit output matches reference Poseidon')

  // Step 3 — verify
  console.log('\nverifying proof against verification_key.json...')
  const ok = await snarkjs.groth16.verify(VKEY, publicSignals, proof)
  if (!ok) throw new Error('FAIL: proof did not verify')
  console.log('  ✓ proof verifies')

  // Step 4 — negative case: tamper with public signal, must reject
  console.log('\ntamper test (changing commitment_hash by +1)...')
  const tampered = [(BigInt(publicSignals[0]) + 1n).toString()]
  const okTampered = await snarkjs.groth16.verify(VKEY, tampered, proof)
  if (okTampered) throw new Error('FAIL: tampered proof was accepted')
  console.log('  ✓ tampered proof correctly rejected')

  // Step 5 — export Solidity / on-chain verifier format
  console.log('\nexporting calldata for an on-chain verifier...')
  const calldata = await snarkjs.groth16.exportSolidityCallData(proof, publicSignals)
  // Stash a short prefix for sanity
  console.log('  calldata prefix:', calldata.slice(0, 80) + '...')

  console.log('\n=== ALL CHECKS PASSED ===')
  process.exit(0)
}

main().catch((e) => {
  console.error('\nFAIL:', e)
  process.exit(1)
})
