"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { listInvoices, type Invoice } from "@/lib/api";

export default function InvoicesPage() {
  const [items, setItems] = useState<Invoice[] | null>(null);
  const [err, setErr] = useState<string>("");

  useEffect(() => {
    (async () => {
      try {
        const data = await listInvoices();
        setItems(data);
      } catch (e:any) {
        setErr(e.message || String(e));
      }
    })();
  }, []);

  return (
    <main className="card">
      <h2>Invoices</h2>
      {err && <div className="badge status-error">Error: {err}</div>}
      {!items && !err && <div className="helper">Loading…</div>}
      {items?.length === 0 && <div className="helper">No invoices yet.</div>}
      {items && items.length > 0 && (
        <table>
          <thead>
            <tr>
              <th>Created</th>
              <th>Amount (USDC)</th>
              <th>Recipient</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            {items.map((x) => (
              <tr key={x.id}>
                <td><Link href={`/invoice/${x.id}`}>{new Date(x.created_at).toLocaleString()}</Link></td>
                <td>{x.amount_usdc.toFixed(2)}</td>
                <td><code>{x.recipient.slice(0,4)}…{x.recipient.slice(-4)}</code></td>
                <td>
                  <span className={"badge " + (x.status === "paid" ? "status-paid" : x.status === "pending" ? "status-pending" : "status-error")}>
                    {x.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
