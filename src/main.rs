use anyhow::{anyhow, Context, Result};
use bs58;
use clap::{ArgAction, Parser};
use rust_decimal::Decimal;
use std::borrow::Cow;
use url::form_urlencoded;

/// USDC mints
const USDC_MAINNET: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const USDC_DEVNET:  &str = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";

/// Build a Solana Pay transfer link:
/// solana:<recipient>?amount=<amount>&spl-token=<mint>&reference=<ref>&label=...&message=...&memo=...
#[derive(Parser, Debug)]
#[command(name="solana-pay-link", version, about="Build a Solana Pay transfer URL")]
struct Opts {
    /// Recipient public key (base58, 32 bytes)
    #[arg(long)]
    recipient: String,

    /// Decimal amount (e.g., 12.34). If omitted, wallets will prompt.
    #[arg(long)]
    amount: Option<String>,

    /// SPL token mint (base58). If omitted, it's a native SOL request.
    #[arg(long, conflicts_with_all = ["usdc_mainnet", "usdc_devnet"])]
    spl_token: Option<String>,

    /// Use USDC on mainnet
    #[arg(long, action=ArgAction::SetTrue, conflicts_with_all = ["spl_token", "usdc_devnet"])]
    usdc_mainnet: bool,

    /// Use USDC on devnet (testnet mint)
    #[arg(long, action=ArgAction::SetTrue, conflicts_with_all = ["spl_token", "usdc_mainnet"])]
    usdc_devnet: bool,

    /// One or more reference keys (base58, 32 bytes). Repeat flag to add multiple.
    #[arg(long = "reference", action=ArgAction::Append)]
    references: Vec<String>,

    /// Optional label shown by wallets (e.g., your brand)
    #[arg(long)]
    label: Option<String>,

    /// Optional message (e.g., "Invoice 123")
    #[arg(long)]
    message: Option<String>,

    /// Optional memo (stored on-chain in Memo instruction)
    #[arg(long)]
    memo: Option<String>,

    /// Enforce max decimals for `amount` (e.g., 9 for SOL, 6 for USDC). If omitted, no check.
    #[arg(long)]
    max_decimals: Option<u32>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // Validate recipient (32 bytes)
    validate_32_byte_b58(&opts.recipient).context("invalid recipient")?;

    // Determine token mint if any
    let spl_token = if opts.usdc_mainnet {
        Some(USDC_MAINNET.to_string())
    } else if opts.usdc_devnet {
        Some(USDC_DEVNET.to_string())
    } else {
        opts.spl_token.clone()
    };
    if let Some(ref mint) = spl_token {
        validate_32_byte_b58(mint).context("invalid spl-token mint")?;
    }

    // Validate references (each must decode to 32 bytes)
    for r in &opts.references {
        validate_32_byte_b58(r).with_context(|| format!("invalid reference: {r}"))?;
    }

    // Parse and validate amount formatting (no floats, no sci notation)
    let amount_str = if let Some(a) = opts.amount.as_deref() {
        // Ensure canonical decimal formatting and leading zero if < 1
        let dec = Decimal::from_str_exact(a)
            .with_context(|| format!("amount is not a valid decimal: {a}"))?;
        if dec.is_sign_negative() {
            return Err(anyhow!("amount must be non-negative"));
        }
        if let Some(max) = opts.max_decimals {
            let scale = dec.scale();
            if scale > max {
                return Err(anyhow!(
                    "amount has {} decimal places; exceeds max_decimals={}",
                    scale,
                    max
                ));
            }
        }
        Some(dec.to_string())
    } else {
        None
    };

    // Build query string
    let mut pairs: Vec<(&str, Cow<'_, str>)> = Vec::new();
    if let Some(a) = amount_str.as_deref() {
        pairs.push(("amount", Cow::Borrowed(a)));
    }
    if let Some(mint) = spl_token.as_deref() {
        pairs.push(("spl-token", Cow::Borrowed(mint)));
    }
    for r in &opts.references {
        pairs.push(("reference", Cow::Borrowed(r)));
    }
    if let Some(ref s) = opts.label { pairs.push(("label", Cow::Borrowed(s))); }
    if let Some(ref s) = opts.message { pairs.push(("message", Cow::Borrowed(s))); }
    if let Some(ref s) = opts.memo { pairs.push(("memo", Cow::Borrowed(s))); }

    let mut url = format!("solana:{}", opts.recipient);
    if !pairs.is_empty() {
        let qs = form_urlencoded::Serializer::new(String::new())
            .extend_pairs(pairs)
            .finish();
        url.push('?');
        url.push_str(&qs);
    }

    println!("{url}");
    Ok(())
}

fn validate_32_byte_b58(s: &str) -> Result<()> {
    let bytes = bs58::decode(s).into_vec().context("base58 decode failed")?;
    if bytes.len() != 32 {
        return Err(anyhow!("expected 32 bytes, got {}", bytes.len()));
    }
    Ok(())
}
