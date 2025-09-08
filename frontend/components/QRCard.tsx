"use client";

import React from "react";
import QRCode from "react-qr-code";

export default function QRCard({ url }: { url: string }) {
  return (
    <div className="card" style={{ display: "grid", gap: 12, placeItems: "center" }}>
      <QRCode value={url} size={220} />
      <div style={{ wordBreak: "break-all", textAlign: "center" }}>
        <code>{url}</code>
      </div>
      <div className="helper">Scan with a Solana Pay-enabled wallet.</div>
    </div>
  );
}
