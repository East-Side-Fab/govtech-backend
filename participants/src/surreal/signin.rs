#[derive(serde::Serialize)]
pub struct SigninRequest {
    #[serde(rename = "ns")]
    pub namespace: String,
    #[serde(rename = "db")]
    pub database: String,
    pub user: String,
    #[serde(rename = "pass")]
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct SigninResponse {
    pub code: u16,
    pub details: String,
    pub token: String,
}
