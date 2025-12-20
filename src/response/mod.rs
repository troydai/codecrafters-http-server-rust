use crate::consts::CRLF;
use anyhow::{Result, bail};
use std::io::Write;

#[derive(Debug)]
pub struct Response {
    protocol: String,
    status_code: String,
    status_phrase: String,
    headers: Vec<String>,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(protocol: &str, status_code: String, status_phrase: String) -> Self {
        Self {
            protocol: String::from(protocol),
            status_code,
            status_phrase,
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn with_body(protocol: &str, body: &str) -> Self {
        let headers = vec![
            String::from("Content-Type: text/plain"),
            format!("Content-Length: {}", body.len()),
        ];

        Self {
            protocol: String::from(protocol),
            status_code: String::from("200"),
            status_phrase: String::from("OK"),
            headers,
            body: Some(Vec::from(body.as_bytes())),
        }
    }

    pub fn write(&self, stream: &mut impl Write) -> Result<()> {
        let head = format!(
            "{} {} {}",
            self.protocol, self.status_code, self.status_phrase
        );

        stream.write_all(head.as_bytes())?;
        stream.write_all(CRLF)?;

        if let Err(e) = self.headers.iter().try_for_each(|h| -> Result<()> {
            stream.write_all(h.as_bytes())?;
            stream.write_all(CRLF)?;
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
