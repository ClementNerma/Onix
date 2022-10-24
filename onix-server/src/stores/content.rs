use anyhow::{anyhow, bail, Context, Result};
use async_graphql::SimpleObject;
use miniz_oxide::{deflate::compress_to_vec_zlib, inflate::decompress_to_vec_zlib};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{apps::AppTemplate, utils::time::get_now};

pub static MAX_STORE_SIZE_MB: usize = 10;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct StoreContent {
    apps: Vec<AppTemplate>,
    created_on: OffsetDateTime,
}

impl StoreContent {
    pub fn new(apps: Vec<AppTemplate>) -> Self {
        Self {
            apps,
            created_on: get_now(),
        }
    }

    pub fn list_apps(&self) -> &[AppTemplate] {
        &self.apps
    }

    pub fn compress(&self) -> Result<Vec<u8>> {
        let ser = serde_json::to_string(self)
            .context("Failed to serialize the provided store as JSON")?;

        let compressed = compress_to_vec_zlib(ser.as_bytes(), 10);

        if compressed.len() > MAX_STORE_SIZE_MB * 1024 * 1024 {
            bail!("The store is too big to be encoded :(");
        }

        Ok(compressed)
    }

    pub fn decompress(compressed: &[u8]) -> Result<Self> {
        if compressed.len() > MAX_STORE_SIZE_MB * 1024 * 1024 {
            bail!("Provided packed store is too big ({} bytes), anti-DDOS triggered with max limit being {} bytes", compressed.len(), MAX_STORE_SIZE_MB * 1024 * 1024);
        }

        let bytes = decompress_to_vec_zlib(compressed)
            .map_err(|e| anyhow!("Failed to decompress the packed store: {e}"))?;

        let str =
            std::str::from_utf8(&bytes).context("Failed to decode the packed store as UTF-8")?;

        serde_json::from_str(&str).context("Failed to decode the packed store")
    }
}
