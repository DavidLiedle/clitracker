use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub id: u64,
    pub name: String,
    #[serde(rename = "current_state")]
    pub current_state: String,
}
