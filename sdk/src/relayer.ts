export interface RelayerInfo {
  id: string
  endpoint: string
  staked_amount: string
  reputation: number
}

export async function listRelayers(api: string): Promise<RelayerInfo[]> {
  const res = await fetch(`${api}/v1/relayers`, {
    headers: { accept: "application/json" },
  })
  if (!res.ok) throw new Error(`failed to fetch relayers: ${res.status}`)
  return res.json() as Promise<RelayerInfo[]>
}

// rev-zn1520

// rev-vqg3bw

// rev-b4sikg

// rev-vziyss

// rev-psh5b2

// rev-9tg1tq

// rev-wk8kdv

// rev-e7nlke

// rev-gsq9ib

// rev-ydh50t
