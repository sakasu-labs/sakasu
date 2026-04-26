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

// rev-uyeuau

// rev-0gevyl

// rev-ba9v6y

// rev-9atz51

// rev-up0u53

// rev-rf1d94

// rev-b7dl6e

// rev-m6xnof

// rev-acomr5

// rev-usk8l0

// rev-xhdjdp

// rev-5ao8b4

// rev-l8mche

// rev-qgih0t

// rev-94gklr

// rev-m26h6e

// rev-u3bisn

// rev-dk4cfr
