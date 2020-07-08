use serde::Serialize;
use serde_json::Result;

#[derive(Serialize)]
pub struct Service {
    pub id: String,
    pub title: String,
    pub subtitle: String,
}

impl Service {
    pub fn serialize_to_json(self) -> Result<String> {
        serde_json::to_string(&self)
    }
}
