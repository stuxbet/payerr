use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct LinkRequest {
    /// Optional; if omitted, server uses RECIPIENT_PUBKEY env var.
    pub(crate) recipient: Option<String>,
    /// Decimal amount as string (e.g., "12.34"). Optional.
    pub(crate)amount: Option<String>,
    /// Explicit mint (base58, 32 bytes). Conflicts with use_usdc_mainnet/use_usdc_devnet.
    pub(crate) spl_token: Option<String>,
    /// Convenience flags
    pub(crate) use_usdc_mainnet: Option<bool>,
    pub(crate) use_usdc_devnet: Option<bool>,
    /// Optional label/message/memo for wallet UI
    pub(crate) label: Option<String>,
    pub(crate) message: Option<String>,
    pub(crate) memo: Option<String>,
    /// Enforce decimal places (e.g., 9 for SOL, 6 for USDC)
    pub(crate) max_decimals: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct LinkResponse {
    pub(crate) url: String,
    pub(crate) reference: String,
}