const DOMAIN_TAG = new TextEncoder().encode("SAKASU_ZK_V0001 ")

export interface CommitmentInput {
  chainId: number
  asset: string
  amount: bigint
  viewKey: Uint8Array
}

/** Build a Sakasu shielded commitment hex string.
 *  Uses SubtleCrypto SHA-256 (browser + Node 19+ runtimes), serving as a
 *  stand-in for the on-chain Poseidon hash used inside the circuit. */
export async function buildCommitment(input: CommitmentInput): Promise<string> {
  if (input.asset.length === 0 || input.asset.length > 16) {
    throw new Error("asset symbol must be 1-16 chars")
  }
  const buf = new Uint8Array(
    DOMAIN_TAG.length + 2 + 16 + 16 + input.viewKey.length,
  )
  let o = 0
  buf.set(DOMAIN_TAG, o); o += DOMAIN_TAG.length
  new DataView(buf.buffer).setUint16(o, input.chainId, true); o += 2
  buf.set(new TextEncoder().encode(input.asset.padEnd(16, "\0")).slice(0, 16), o); o += 16
  const amt = input.amount
  for (let i = 0; i < 16; i++) {
    buf[o + i] = Number((amt >> BigInt(i * 8)) & 0xffn)
  }
  o += 16
  buf.set(input.viewKey, o)
  const digest = await crypto.subtle.digest("SHA-256", buf)
  return [...new Uint8Array(digest)]
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("")
}

export function generateViewKey(): Uint8Array {
  const k = new Uint8Array(32)
  crypto.getRandomValues(k)
  return k
}

// rev-u3fylt

// rev-yn6im1

// rev-bplm15

// rev-5l49nm

// rev-xzp34z

// rev-8ebpqe

// rev-oik18n

// rev-459s3w

// rev-wtps78

// rev-y0noxu
