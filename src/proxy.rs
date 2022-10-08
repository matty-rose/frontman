use std::time::Duration;

use rand::seq::SliceRandom;

use crate::config::Config;
use anyhow::Result;
use axum::{extract::Path, http};
use reqwest::Client;

#[derive(Clone)]
pub struct Proxy {
    config: Config,
    client: Client,
}

impl Proxy {
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.frontman.timeout))
            .build()
            .unwrap();

        Self { config, client }
    }

    pub async fn handler(&self, Path(path): Path<String>, method: http::Method) -> Result<String> {
        let origin = self.get_origin();
        tracing::info!(
            "forwarding {} request to path {} to this server: {}",
            method,
            path,
            origin
        );

        let resp = self
            .client
            .request(method, format!("{}{}", origin, path))
            .send()
            .await?
            .text()
            .await?;
        tracing::info!("{:?}", resp);

        Ok(resp)
    }

    fn get_origin(&self) -> String {
        let origin = self
            .config
            .frontman
            .origins
            .choose(&mut rand::thread_rng())
            .unwrap();
        format!("http://{}:{}", origin.host, origin.port)
    }
}
