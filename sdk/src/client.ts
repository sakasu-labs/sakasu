import { buildCommitment, generateViewKey } from "./commitment.js"
import type { Intent } from "./intent.js"

export interface SakasuClientOptions {
  endpoint: string
}

export interface ShieldedTransferInput {
  fromChain: "solana" | "base" | "ethereum" | "arbitrum"
  toChain: "solana" | "base" | "ethereum" | "arbitrum"
  asset: string
  amount: bigint
}

export class SakasuClient {
  constructor(private readonly opts: SakasuClientOptions) {}

  async buildShieldedTransfer(input: ShieldedTransferInput): Promise<Intent> {
    const viewKey = generateViewKey()
    const commitment = await buildCommitment({
      chainId: chainCode(input.fromChain),
      asset: input.asset,
      amount: input.amount,
      viewKey,
    })
    return {
      version: 2,
      fromChain: input.fromChain,
      toChain: input.toChain,
      asset: input.asset,
      amount: input.amount,
      commitment,
      viewKey,
      deadlineSlot: 0n,
    }
  }

  async submitIntent(intent: Intent): Promise<{ commitmentHash: string }> {
    const payload = {
      version: intent.version,
      fromChain: intent.fromChain,
      toChain: intent.toChain,
      asset: intent.asset,
      amount: intent.amount.toString(),
      commitment: intent.commitment,
    }
    const res = await fetch(`${this.opts.endpoint}/v1/transfers/initiate`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(payload),
    })
    if (!res.ok) {
      throw new Error(`relayer rejected intent: ${res.status}`)
    }
    return res.json() as Promise<{ commitmentHash: string }>
  }
}

export function chainCode(c: ShieldedTransferInput["fromChain"]): number {
  switch (c) {
    case "solana":   return 101
    case "ethereum": return 1
    case "base":     return 8453
    case "arbitrum": return 42161
  }
}

// rev-9g7upw

// rev-tey3nq

// rev-bjrdiq
