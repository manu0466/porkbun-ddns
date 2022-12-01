use crate::porkbun::requests::{Auth, CreateRecord};
use crate::porkbun::responses::{
    CrateRecordResponse, DeleteRecordResponse, PingResponse, RetrieveRecordsResponse,
    SSLRetrieveBundleResponse,
};
use eyre::{eyre, Context, Result};
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json;
use std::borrow::Cow;

pub struct PorkbunClient {
    api_key: Cow<'static, str>,
    api_secret: Cow<'static, str>,
    endpoint: String,
}

impl PorkbunClient {
    pub fn new<'a>(api_key: Cow<'static, str>, api_secret: Cow<'static, str>) -> Self {
        PorkbunClient {
            api_key,
            api_secret,
            endpoint: "https://porkbun.com/api/json/v3".to_string(),
        }
    }

    fn auth(&self) -> Auth {
        Auth {
            secret_api_key: self.api_secret.clone(),
            api_key: self.api_key.clone(),
        }
    }

    fn send<T>(&self, request: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = request.send().context("error sending the request")?;

        if response.status() != StatusCode::from_u16(200).unwrap() {
            Err(eyre!(
                "Request error: {}, body: {}",
                response.status(),
                response.text().unwrap_or_default()
            ))
        } else {
            response.json().context("deserialization error")
        }
    }

    pub fn ping(&self) -> Result<PingResponse> {
        let request = Client::new()
            .post(format!("{}/ping", self.endpoint))
            .body(serde_json::to_string(&self.auth()).context("auth serialization")?);

        self.send(request).context("ping")
    }

    pub fn retrieve_records(
        &self,
        domain: &str,
        id: Option<String>,
    ) -> Result<RetrieveRecordsResponse> {
        let mut url = format!("{}/dns/retrieve/{}", self.endpoint, domain);
        if let Some(id) = id {
            url = format!("{}/{}", url, id);
        }

        let request = Client::new()
            .post(url)
            .body(serde_json::to_string(&self.auth()).context("auth serialization")?);

        self.send(request).context("retrieve records")
    }

    pub fn delete_record_by_domain_and_id(
        &self,
        domain: &str,
        id: &str,
    ) -> Result<DeleteRecordResponse> {
        let request = Client::new()
            .post(format!("{}/dns/delete/{}/{}", self.endpoint, domain, id))
            .body(serde_json::to_string(&self.auth()).context("auth serialization")?);
        self.send(request).context("delete record")
    }

    pub fn create_record(
        &self,
        domain: &str,
        name: &str,
        record_type: &str,
        content: &str,
        ttl: &str,
    ) -> Result<CrateRecordResponse> {
        let request = Client::new()
            .post(format!("{}/dns/create/{}", self.endpoint, domain))
            .body(
                serde_json::to_string(&CreateRecord {
                    auth: self.auth(),
                    name: name.to_string(),
                    record_type: record_type.to_string(),
                    content: content.to_string(),
                    ttl: ttl.to_string(),
                })
                .context("create record serialization")?,
            );
        self.send(request).context("create record")
    }

    pub fn ssl_retrieve_bundle_by_domain(&self, domain: &str) -> Result<SSLRetrieveBundleResponse> {
        let request = Client::new()
            .post(format!("{}/ssl/retrieve/{}", self.endpoint, domain))
            .body(serde_json::to_string(&self.auth())?);
        self.send(request).context("retrieve ssl bundle")
    }
}
