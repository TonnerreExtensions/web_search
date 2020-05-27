use serde::Serialize;

#[derive(Serialize)]
pub struct Service {
    pub id: String,
    pub title: String,
    pub subtitle: String,
}
