use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug)]
pub struct Auth {
    #[serde(rename = "secretapikey")]
    pub secret_api_key: Cow<'static, str>,
    #[serde(rename = "apikey")]
    pub api_key: Cow<'static, str>,
}

#[derive(Serialize, Debug)]
pub struct CreateRecord {
    #[serde(flatten)]
    pub auth: Auth,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: String,
    pub ttl: String,
}
