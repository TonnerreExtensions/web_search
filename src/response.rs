use serde::Serialize;
use serde_json::Result;

#[derive(Serialize)]
pub struct Response<'a, S: Serialize> {
    provider: &'a str,
    services: Vec<S>,
}

impl<'a, S: Serialize> Response<'a, S> {
    pub fn new(identifier: &'a str, services: Vec<S>) -> Self {
        Response {
            provider: identifier,
            services,
        }
    }

    pub fn serialize_to_json(self) -> Result<String> {
        serde_json::to_string(&self)
    }
}
