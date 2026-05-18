export interface Intent {
  version: number
  fromChain: string
  toChain: string
  asset: string
  amount: bigint
  commitment: string
  viewKey: Uint8Array
  deadlineSlot: bigint
}

export interface EncryptedIntent {
  ciphertext: string
  nonce: string
  ephemeral_pubkey: string
}

/** Encrypt an intent for relayer eyes only.
 *  Placeholder: base64-encodes the JSON view of the intent. The real
 *  implementation uses X25519 ECDH + ChaCha20-Poly1305 (ships in v0.3). */
export function encryptIntent(intent: Intent): EncryptedIntent {
  const view = {
    v: intent.version,
    f: intent.fromChain,
    t: intent.toChain,
    a: intent.asset,
    n: intent.amount.toString(),
    c: intent.commitment,
    d: intent.deadlineSlot.toString(),
  }
  const json = JSON.stringify(view)
  const base64 =
    typeof Buffer !== "undefined"
      ? Buffer.from(json, "utf-8").toString("base64")
      : btoa(json)
  return { ciphertext: base64, nonce: "", ephemeral_pubkey: "" }
}






















