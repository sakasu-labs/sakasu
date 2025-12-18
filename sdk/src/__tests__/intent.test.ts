import { describe, it, expect } from "vitest"
import { encryptIntent, type Intent } from "../intent.js"

describe("intent", () => {
  it("encrypts to a non-empty ciphertext", () => {
    const intent: Intent = {
      version: 2,
      fromChain: "solana",
      toChain: "base",
      asset: "USDC",
      amount: 250n,
      commitment: "deadbeef",
      viewKey: new Uint8Array(32),
      deadlineSlot: 0n,
    }
    const enc = encryptIntent(intent)
    expect(enc.ciphertext.length).toBeGreaterThan(0)
  })
})
