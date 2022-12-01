use eyre::Context;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct SslConfig {
    pub path: String,
    pub certificate_chain: Option<String>,
    pub intermediate_certificate: Option<String>,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub api_secret: String,
    pub domain: String,
    pub sub_domains: Vec<String>,
    pub ssl: SslConfig,
}

impl Config {
    pub fn from_yaml(path: &str) -> eyre::Result<Self> {
        let yaml_file = File::open(path).context("open yaml config file")?;
        serde_yaml::from_reader::<File, Config>(yaml_file).context("parsing yaml file")
    }
}
