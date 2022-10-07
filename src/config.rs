use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub frontman: FrontmanConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FrontmanConfig {
    pub origins: Vec<Origin>,
    pub timeout: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Origin {
    pub host: String,
    pub port: usize,
}
