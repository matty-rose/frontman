use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub frontman: FrontmanConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FrontmanConfig {
    pub origins: Vec<Origin>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Origin {
    host: String,
    port: usize,
}
