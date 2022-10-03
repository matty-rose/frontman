use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub frontman: FrontmanConfig,
}

#[derive(Deserialize, Debug)]
pub struct FrontmanConfig {
    pub origins: Vec<Origin>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Origin {
    uri: String,
}
