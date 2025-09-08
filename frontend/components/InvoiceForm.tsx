"use client";

import React, { useState } from "react";
import { createInvoice } from "@/lib/api";
import { useRouter } from "next/navigation";

export default function InvoiceForm() {
  const [amount, setAmount] = useState<string>("");
  const [recipient, setRecipient] = useState<string>("");
  const [email, setEmail] = useState<string>("");
  const [memo, setMemo] = useState<string>("");
  const [label, setLabel] = useState<string>("");
  const [message, setMessage] = useState<string>("");
  const [busy, setBusy] = useState(false);
  const [err, setErr] = useState<string>("");
  const router = useRouter();

  async function onSubmit(e: React.FormEvent) {
    e.preventDefault();
    setBusy(true);
    setErr("");
    try {
      const amt = parseFloat(amount);
      if (!isFinite(amt) || amt <= 0) throw new Error("Amount must be > 0");
      if (recipient.length < 20) throw new Error("Recipient must be a valid Solana public key");
      const inv = await createInvoice({
        amount_usdc: amt,
        recipient,
        memo: memo || undefined,
        label: label || undefined,
        message: message || undefined,
        email: email || undefined
      });
      router.push(`/invoice/${inv.id}`);
    } catch (e:any) {
      setErr(e.message || String(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <form onSubmit={onSubmit} className="card" style={{ display: "grid", gap: 12 }}>
      <div>
        <div className="label">Amount (USDC)</div>
        <input className="input" placeholder="100.00" inputMode="decimal" value={amount} onChange={e => setAmount(e.target.value)} />
        <div className="helper">Exact amount required; invoice auto-matches on-chain transfer.</div>
      </div>
      <div>
        <div className="label">Recipient (Merchant Wallet)</div>
        <input className="input" placeholder="Merchant public key" value={recipient} onChange={e => setRecipient(e.target.value)} />
      </div>
      <div className="row">
        <div>
          <div className="label">Label (optional)</div>
          <input className="input" placeholder="Acme LLC" value={label} onChange={e => setLabel(e.target.value)} />
        </div>
        <div>
          <div className="label">Memo (optional)</div>
          <input className="input" placeholder="INV-1042" value={memo} onChange={e => setMemo(e.target.value)} />
        </div>
      </div>
      <div>
        <div className="label">Message (optional)</div>
        <input className="input" placeholder="Invoice #1042" value={message} onChange={e => setMessage(e.target.value)} />
      </div>
      <div>
        <div className="label">Receipt Email (optional)</div>
        <input className="input" placeholder="you@company.com" value={email} onChange={e => setEmail(e.target.value)} />
      </div>
      {err && <div className="badge status-error">Error: {err}</div>}
      <div style={{ display: "flex", gap: 12, justifyContent: "flex-end" }}>
        <button className="btn" type="button" onClick={() => { setAmount(""); setRecipient(""); setMemo(""); setLabel(""); setMessage(""); setEmail(""); }}>
          Clear
        </button>
        <button className="btn btn-primary" disabled={busy}>
          {busy ? "Creating..." : "Create invoice"}
        </button>
      </div>
    </form>
  );
}
