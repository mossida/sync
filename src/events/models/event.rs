use serde::Serialize;

#[derive(Serialize)]
pub struct Event {
    pub name: String,
}