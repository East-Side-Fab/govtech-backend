#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: Name,
    pub mail: String,
    pub organisation: Option<String>,
    #[serde(default)]
    pub receive_govtech_news: bool,
    #[serde(default)]
    pub receive_esf_news: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Name {
    pub first: String,
    pub last: String,
}
