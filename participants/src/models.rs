#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: Name,
    pub mail: String,
    pub organisation: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Name {
    pub first: String,
    pub last: String,
}
