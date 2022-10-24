mod content;

use anyhow::{anyhow, Context, Result};
use async_graphql::{InputObject, SimpleObject};

pub use self::content::StoreContent;

#[derive(SimpleObject, InputObject)]
#[graphql(input_name_suffix = "Input")]
pub struct StoreConfig {
    url: String,
}

pub struct StoreInterface {
    url: String,
}

impl StoreInterface {
    pub fn new(config: StoreConfig) -> Self {
        Self { url: config.url }
    }

    pub async fn pull(&self) -> Result<StoreContent> {
        let resp = reqwest::get(&self.url)
            .await
            .context("Failed to GET packed store")?;

        let resp = resp
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to GET packed store as bytes: {e}"))?;

        StoreContent::decompress(&resp)
    }
}
