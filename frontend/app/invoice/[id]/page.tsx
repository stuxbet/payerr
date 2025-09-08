"use client";

import { useEffect, useMemo, useState } from "react";
import { useParams } from "next/navigation";
import { getInvoice, type Invoice } from "@/lib/api";
import QRCard from "@/components/QRCard";

export default function InvoiceDetail() {
  const params = useParams<{ id: string }>();
  const id = params.id;
  const [inv, setInv] = useState<Invoice | null>(null);
  const [err, setErr] = useState<string>("");

  useEffect(() => {
    let alive = true;
    async function tick() {
      try {
        const data = await getInvoice(id);
        if (!alive) return;
        setInv(data);
      } catch (e:any) {
        if (!alive) return;
        setErr(e.message || String(e));
      }
    }
    tick();
    const h = setInterval(tick, 2500); // poll every 2.5s
    return () => { alive = false; clearInterval(h); };
  }, [id]);

  const url = useMemo(() => inv?.solana_pay_url ?? "", [inv]);

  return (
    <main className="row row-2">
      <section className="card">
        <h2>Invoice #{id}</h2>
        {err && <div className="badge status-error">Error: {err}</div>}
        {!inv && !err && <div className="helper">Loading…</div>}
        {inv && (
          <div style={{ display:"grid", gap: 12 }}>
            <div className="badge">Recipient <code>{inv.recipient}</code></div>
            <div className="badge">Amount <strong>{inv.amount_usdc.toFixed(2)}</strong> USDC</div>
            <div className="badge">Reference <code>{inv.reference}</code></div>
            <div className={"badge " + (inv.status === "paid" ? "status-paid" : inv.status === "pending" ? "status-pending" : "status-error")}>
              Status: {inv.status}
            </div>
            {inv.memo && <div className="badge">Memo <code>{inv.memo}</code></div>}
            {inv.due_at && <div className="badge">Due by {new Date(inv.due_at).toLocaleString()}</div>}
          </div>
        )}
      </section>
      <section>
        {url ? <QRCard url={url} /> : <div className="card"><div className="helper">Awaiting payment link…</div></div>}
        {inv?.status === "paid" && (
          <div className="card" style={{ marginTop: 12 }}>
            <h3>Paid ✅</h3>
            <p className="helper">A receipt should arrive via email if provided.</p>
          </div>
        )}
      </section>
    </main>
  );
}
