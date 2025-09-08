export type LinkResponse = {
  url: string;
  reference: string;
};

const BASE = process.env.NEXT_PUBLIC_API_BASE || "http://localhost:8787";

async function ok<T>(res: Response): Promise<T> {
  if (!res.ok) {
    const text = await res.text().catch(() => "");
    throw new Error(`API ${res.status}: ${text}`);
  }
  return res.json();
}

export function clusterFromEnv(): "devnet" | "mainnet-beta" {
  const c = (process.env.NEXT_PUBLIC_SOLANA_CLUSTER || "devnet") as any;
  return c === "mainnet-beta" ? "mainnet-beta" : "devnet";
}

export async function createLink(input: {
  amount_usdc: number;
  recipient: string;
  memo?: string;
  label?: string;
  message?: string;
}): Promise<LinkResponse> {
  // Backend expects decimal string `amount` and a USDC mint flag
  const cluster = clusterFromEnv();
  const body = {
    recipient: input.recipient,
    amount: input.amount_usdc.toFixed(2),
    memo: input.memo,
    label: input.label,
    message: input.message,
    use_usdc_mainnet: cluster === "mainnet-beta",
    use_usdc_devnet: cluster === "devnet"
  };
  const res = await fetch(`${BASE}/api/link`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body)
  });
  return ok<LinkResponse>(res);
}
