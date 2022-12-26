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

/// HTTP Post body for the [DNS Edit Record by Domain and ID](https://porkbun.com/api/json/v3/documentation#DNS%20Edit%20Record%20by%20Domain%20and%20ID)
/// request.
#[derive(Serialize, Debug)]
pub struct EditRecord {
    #[serde(flatten)]
    pub auth: Auth,
    /// The subdomain for the record being created,
    /// not including the domain itself.
    /// Leave blank to create a record on the root domain.
    /// Use * to create a wildcard record
    pub name: String,
    /// The type of record being created.
    /// Valid types are: A, MX, CNAME, ALIAS, TXT, NS, AAAA, SRV, TLSA, CAA
    #[serde(rename = "type")]
    pub record_type: String,
    /// The answer content for the record.
    /// Please see the DNS management popup from the domain management
    /// console for proper formatting of each record type.
    pub content: String,
    /// The time to live in seconds for the record.
    /// The minimum and the default is 600 seconds.
    pub ttl: Option<String>,
    /// The priority of the record for those that support it.
    pub prio: Option<String>,
}
