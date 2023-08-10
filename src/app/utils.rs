use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub status: String,
    pub message: String,
}