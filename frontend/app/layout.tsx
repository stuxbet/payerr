import "./globals.css";
import type { Metadata } from "next";
import Link from "next/link";
import WalletCtx from "../components/WalletCtx";

export const metadata: Metadata = {
  title: "USDC Invoices · Solana Pay",
  description: "Generate Solana Pay invoice links and QR codes for USDC."
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <WalletCtx>
          <div className="container">
            <header className="header">
              <div className="brand">
                <span style={{ fontSize: 22 }}>💸</span>
                <Link href="/">USDC Invoice Links</Link>
              </div>
              <nav style={{ display: "flex", gap: 12 }}>
                <a className="btn" href="https://docs.solanapay.com" target="_blank" rel="noreferrer">Solana Pay Docs</a>
              </nav>
            </header>
            {children}
            <footer>
              <p>Non-custodial. Uses Solana Pay reference tags to match on-chain payments.</p>
              <p>Backend URL: <code>{process.env.NEXT_PUBLIC_API_BASE}</code> · Cluster: <code>{process.env.NEXT_PUBLIC_SOLANA_CLUSTER}</code></p>
            </footer>
          </div>
        </WalletCtx>
      </body>
    </html>
  );
}
