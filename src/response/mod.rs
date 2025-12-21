use crate::consts::{self, CRLF};
use crate::header::{Header, Headers};
use anyhow::{Result, bail};
use std::io::Write;

#[derive(Debug)]
pub struct Response {
    status_code: String,
    status_phrase: String,
    headers: Headers,
    body: Option<Vec<u8>>,
}

impl Response {
    pub const fn new(status_code: String, status_phrase: String) -> Self {
        Self {
            status_code,
            status_phrase,
            headers: Headers::new(),
            body: None,
        }
    }

    pub fn with_body(body: &str) -> Self {
        let mut headers = Headers::new();
        headers.add_header(Header::new("Content-Type", "text/plain"));
        headers.add_header(Header::new("Content-Length", &body.len().to_string()));

        Self {
            status_code: String::from("200"),
            status_phrase: String::from("OK"),
            headers,
            body: Some(Vec::from(body.as_bytes())),
        }
    }

    pub fn write(&self, stream: &mut impl Write) -> Result<()> {
        let head = format!(
            "{} {} {}",
            consts::STR_HTTP_1_1,
            self.status_code,
            self.status_phrase
        );

        stream.write_all(head.as_bytes())?;
        stream.write_all(CRLF)?;

        if let Err(e) = self.headers.vec().iter().try_for_each(|h| -> Result<()> {
            stream.write_all(&h.wire_representation())?;
            Ok(())
        }) {
            bail!("failed to write {e} headers")
        }

        // empty line to separate body from headers
        stream.write_all(CRLF)?;
        if let Some(body) = &self.body {
            stream.write_all(body.as_slice())?;
        }

        stream.flush()?;
        Ok(())
    }
}

pub fn internal_err_response() -> Response {
    Response::new(String::from("500"), String::from("Internal Server Error"))
}

pub fn bad_request(body: &str) -> Response {
    Response {
        status_code: String::from("400"),
        status_phrase: String::from("Bad Request"),
        headers: Headers::new(),
        body: Some(Vec::from(body.as_bytes())),
    }
}

pub fn ok() -> Response {
    Response::new(String::from("200"), String::from("OK"))
}

pub fn not_found() -> Response {
    Response::new(String::from("404"), String::from("Not Found"))
}
