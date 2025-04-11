#[derive(Debug, serde::Deserialize)]
pub struct SQLQueryResponse {
    pub status: String,
    pub time: String,
    pub result: serde_json::Value,
}
