use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PingResponse {
    pub status: String,
    #[serde(rename = "yourIp")]
    pub your_ip: String,
}

#[derive(Deserialize, Debug)]
pub struct DnsRecord {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: String,
    pub ttl: String,
    pub prio: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RetrieveRecordsResponse {
    pub status: String,
    pub records: Vec<DnsRecord>,
}

#[derive(Deserialize, Debug)]
pub struct DeleteRecordResponse {
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct CrateRecordResponse {
    pub status: String,
    pub id: u64,
}
