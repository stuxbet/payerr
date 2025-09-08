# USDC Invoice Links (Solana Pay) — Next.js Frontend

Minimal Next.js (App Router, TS) UI for creating invoices that encode a Solana Pay URL + QR and polling status from your Rust backend.

## Features
- Create invoice form: amount, recipient, optional label/message/memo/email
- Shows Solana Pay URL as QR + text
- Polls status (`pending` → `paid`) on the invoice detail page
- Simple invoices list page
- Wallet context pre-wired (Phantom, Solflare, Coinbase, Backpack) for future UX

> Assumes the backend exposes:
>
> - `POST /invoices` → `{ id, amount_usdc, recipient, reference, status, created_at, solana_pay_url }`
> - `GET /invoices` → `Invoice[]`
> - `GET /invoices/:id` → `Invoice`
> - `GET /invoices/:id/receipt` → (optional, not used in UI yet)
>
> Adjust `lib/api.ts` if your routes differ.

## Quickstart
```bash
# 1) Copy env and adjust backend base URL
cp .env.local.example .env.local
# Example: http://localhost:8787 for Axum
# NEXT_PUBLIC_API_BASE=http://localhost:8787
# NEXT_PUBLIC_SOLANA_CLUSTER=devnet

# 2) Install deps
pnpm i    # or: npm i / yarn

# 3) Run dev server
pnpm dev  # open http://localhost:3000
```

## Notes
- QR is rendered client-side; wallets parse the Solana Pay URL to build and send the transfer.
- The backend must generate a unique `reference` and verify incoming USDC transfers match `amount/recipient/mint/reference` with **finalized** commitment.
- UI is intentionally minimal (no auth, no plan gating). Add your branding/flows on top.
