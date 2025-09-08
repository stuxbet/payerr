export type Invoice = {
  id: string;
  merchant_id: string;
  amount_usdc: number;
  recipient: string;
  reference: string;
  status: "pending" | "paid" | "expired" | "error";
  due_at?: string | null;
  memo?: string | null;
  created_at: string;
  solana_pay_url?: string;
};

const BASE = process.env.NEXT_PUBLIC_API_BASE || "http://localhost:8787";

async function ok<T>(res: Response): Promise<T> {
  if (!res.ok) {
    const text = await res.text().catch(() => "");
    throw new Error(`API ${res.status}: ${text}`);
  }
  return res.json();
}

export async function createInvoice(input: {
  amount_usdc: number;
  recipient: string;
  memo?: string;
  label?: string;
  message?: string;
  email?: string;
}): Promise<Invoice> {
  const res = await fetch(`${BASE}/invoices`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(input)
  });
  return ok<Invoice>(res);
}

export async function getInvoice(id: string): Promise<Invoice> {
  const res = await fetch(`${BASE}/invoices/${id}`, { cache: "no-store" });
  return ok<Invoice>(res);
}

export async function listInvoices(): Promise<Invoice[]> {
  const res = await fetch(`${BASE}/invoices`, { cache: "no-store" });
  return ok<Invoice[]>(res);
}

export function clusterFromEnv(): "devnet" | "mainnet-beta" {
  const c = (process.env.NEXT_PUBLIC_SOLANA_CLUSTER || "devnet") as any;
  return c === "mainnet-beta" ? "mainnet-beta" : "devnet";
}
