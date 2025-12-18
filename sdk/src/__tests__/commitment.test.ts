import { describe, it, expect } from "vitest"
import { buildCommitment, generateViewKey } from "../commitment.js"

describe("commitment", () => {
  it("is deterministic under identical inputs", async () => {
    const vk = new Uint8Array(32).fill(7)
    const a = await buildCommitment({ chainId: 101, asset: "USDC", amount: 250n, viewKey: vk })
    const b = await buildCommitment({ chainId: 101, asset: "USDC", amount: 250n, viewKey: vk })
    expect(a).toBe(b)
  })

  it("differs across view keys", async () => {
    const a = await buildCommitment({ chainId: 1, asset: "SOL", amount: 1n, viewKey: new Uint8Array(32).fill(1) })
    const b = await buildCommitment({ chainId: 1, asset: "SOL", amount: 1n, viewKey: new Uint8Array(32).fill(2) })
    expect(a).not.toBe(b)
  })

  it("rejects empty asset", async () => {
    const vk = new Uint8Array(32)
    await expect(
      buildCommitment({ chainId: 1, asset: "", amount: 1n, viewKey: vk }),
    ).rejects.toThrow()
  })

  it("generates 32-byte view keys", () => {
    const k = generateViewKey()
    expect(k.length).toBe(32)
  })
})
