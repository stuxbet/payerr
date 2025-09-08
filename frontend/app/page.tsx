import InvoiceForm from "@/components/InvoiceForm";

export default function Page() {
  return (
    <main className="row row-2">
      <section className="card">
        <h2>Create an invoice</h2>
        <p className="helper">Enter the merchant wallet and USDC amount. We generate a Solana Pay URL + QR and watch for payment using a unique reference.</p>
        <InvoiceForm />
      </section>
      <section className="card">
        <h3>How it works</h3>
        <ol>
          <li>We generate a Solana Pay URL with recipient, amount, USDC mint, and a unique <code>reference</code>.</li>
          <li>A wallet scans/clicks the link and sends the USDC.</li>
          <li>Backend detects a transfer with the reference to the recipient for the exact amount, then marks the invoice as paid and emails a receipt.</li>
        </ol>
        <p className="helper">USDC mint (mainnet): <code>EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v</code></p>
      </section>
    </main>
  );
}
