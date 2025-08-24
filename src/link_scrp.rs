use crate::{models::LinkResponse, AppError};

use anyhow::{anyhow, Context, Result};
use axum::{
    extract::Query,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use bs58;
use rand::{distributions::Standard, Rng};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::env;
use url::form_urlencoded;
use crate::models::LinkRequest;

pub async fn create_link(Json(req): Json<LinkRequest>) -> Result<impl IntoResponse, AppError> {
    let recipient = match req.recipient {
        Some(ref r) => r.to_string(),
        None => match env::var("RECIPIENT_PUBKEY") {
            Ok(r) => r,
            Err(_) => return Err(AppError::bad_request("recipient missing (no RECIPIENT_PUBKEY env)")),
        },
    };
    let USDC_MAINNET = match req.recipient {
        Some(ref r) => r.to_string(),
        None => match env::var("USDC_MAINNET") {
            Ok(r) => r,
            Err(_) => return Err(AppError::bad_request("recipient missing (no USDC_MAINNET env)")),
        },
    };
    let USDC_DEVNET = match req.recipient {
        Some(ref r) => r.to_string(),
        None => match env::var("USDC_DEVNET") {
            Ok(r) => r,
            Err(_) => return Err(AppError::bad_request("recipient missing (no USDC_DEVNET env)")),
        },
    };

    let use_main = req.use_usdc_mainnet.unwrap_or(false);
    let use_dev  = req.use_usdc_devnet.unwrap_or(false);

    validate_pubkey(&recipient).context("invalid recipient")?;


    let mint = if let Some(m) = req.spl_token.clone() {
        Some(m)
    } else if use_main {
        Some(USDC_MAINNET.to_string())
    } else if use_dev {
        Some(USDC_DEVNET.to_string())
    } else {
        None // native SOL
    };

    if let Some(m) = &mint {
        validate_pubkey(m).context("invalid spl-token mint")?;
    }

    // Amount (keep exact decimal text; validate formatting)
    let amount_txt = if let Some(a) = req.amount.as_deref() {
        let dec = Decimal::from_str_exact(a).map_err(|e| anyhow!("amount must be a decimal string: {e}"))?;
        if dec.is_sign_negative() { return Err(AppError::bad_request("amount must be non-negative")); }
        if let Some(max) = req.max_decimals { if dec.scale() > max { return Err(AppError::bad_request(&format!("amount has {} decimal places; max {}", dec.scale(), max))); } }
        Some(dec.to_string())
    } else { None };

    // Generate a random 32-byte reference (no private key needed)
    let reference_bytes: [u8; 32] = rand::thread_rng().sample(Standard);
    let reference_b58 = bs58::encode(reference_bytes).into_string();

    // Build query string
    let mut pairs: Vec<(&str, String)> = Vec::new();
    if let Some(a) = amount_txt { pairs.push(("amount", a)); }
    if let Some(m) = &mint { pairs.push(("spl-token", m.clone())); }
    pairs.push(("reference", reference_b58.clone()));
    if let Some(s) = req.label { pairs.push(("label", s)); }
    if let Some(s) = req.message { pairs.push(("message", s)); }
    if let Some(s) = req.memo { pairs.push(("memo", s)); }

    let mut url = format!("solana:{}", recipient);
    if !pairs.is_empty() {
        let qs = form_urlencoded::Serializer::new(String::new())
            .extend_pairs(pairs.iter().map(|(k, v)| (*k, v.as_str())))
            .finish();
        url.push('?');
        url.push_str(&qs);
    }

    Ok((StatusCode::OK, Json(LinkResponse { url, reference: reference_b58 })))
}

#[derive(Debug, Deserialize)]
pub struct QrParams { url: String, size: Option<u32> }

pub async fn qr_png(Query(q): Query<QrParams>) -> Result<Response, AppError> {
    let size = q.size.unwrap_or(512).clamp(128, 2048);
    // Generate QR
    let code = qrcode::QrCode::new(q.url.as_bytes()).map_err(|e| AppError::bad_request(&format!("invalid url for QR: {e}")))?;
    let image = code.render::<qrcode::render::svg::Color>()
        .min_dimensions(size, size)
        .dark_color(qrcode::render::svg::Color("#000"))
        .light_color(qrcode::render::svg::Color("#fff"))
        .build();
    // Return SVG for crisp scaling OR PNG; we'll do PNG below using image buffer
    // For PNG, render to Luma8 buffer first
    let code_png = qrcode::QrCode::new(q.url.as_bytes()).unwrap();
    let scale = (size / code_png.width() as u32).max(1);
    let mut img = image::ImageBuffer::<image::Luma<u8>, Vec<u8>>::new(code_png.width() as u32 * scale, code_png.width() as u32 * scale);
    for y in 0..code_png.width() {
        for x in 0..code_png.width() {
            let color = code_png[(x, y)];
            let dark = color == qrcode::Color::Dark;
            for dy in 0..scale { for dx in 0..scale {
                let px = x as u32 * scale + dx;
                let py = y as u32 * scale + dy;
                img.put_pixel(px, py, if dark { image::Luma([0u8]) } else { image::Luma([255u8]) });
            }}
        }
    }
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);
    image::DynamicImage::ImageLuma8(img).write_to(&mut cursor, image::ImageFormat::Png).map_err(|e| AppError::internal(&format!("png encode: {e}")))?;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/png"));
    Ok((StatusCode::OK, headers, bytes).into_response())
}

pub fn validate_pubkey(s: &str) -> Result<()> {
    let bytes = bs58::decode(s).into_vec().context("base58 decode failed")?;
    if bytes.len() != 32 { return Err(anyhow!("expected 32 bytes, got {}", bytes.len())); }
    Ok(())
}