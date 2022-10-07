use crate::config::Config;
use anyhow::Result;
use axum::extract::Path;

#[derive(Clone)]
pub struct Proxy {
    config: Config,
}

impl Proxy {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    pub async fn handler(&self, Path(path): Path<String>) -> Result<String, ()> {
        Ok(format!(
            "forwarding {} to one of these servers: {:?}",
            path, self.config.frontman.origins
        ))
    }
}
