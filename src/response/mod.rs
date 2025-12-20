#[derive(Debug)]
pub struct Response {
    pub protocol: String,
    pub status_code: String,
    pub status_phrase: String,
}

impl Response {
    pub fn new(protocol: &str, status_code: String, status_phrase: String) -> Self {
        Self {
            protocol: String::from(protocol),
            status_code,
            status_phrase,
        }
    }
}
