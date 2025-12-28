#[cfg(test)]
mod tests;

use crate::body::HttpBody;
use crate::consts::{CRLF, HEADER_CONNECTION, HEADER_CONTENT_ENCODING};
use crate::header::Headers;
use crate::http::status::HttpStatus;
use anyhow::Result;
use std::io::Write;

#[derive(Debug)]
pub struct Response {
    status: HttpStatus,
    headers: Headers,
    body: HttpBody,
}

impl Response {
    pub fn new(status: HttpStatus) -> Self {
        let mut headers = Headers::new();
        headers.set(HEADER_CONNECTION, "keep-alive");

        Self {
            status,
            headers,
            body: HttpBody::Empty,
        }
    }

    pub fn set_str_body(&mut self, body: &str) {
        let bytes = body.as_bytes();
        self.headers.set("Content-Type", "text/plain");
        self.headers.set_content_length(bytes.len());
        self.body = HttpBody::Content(Vec::from(bytes));
    }

    pub fn set_bytes_body(&mut self, content_type: &str, body: &[u8]) {
        self.headers.set("Content-Type", content_type);
        self.headers.set_content_length(body.len());
        self.body = HttpBody::Content(Vec::from(body));
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.set(name, value);
    }

    pub fn set_encoding(&mut self, encoding: &str) {
        self.headers.set(HEADER_CONTENT_ENCODING, encoding);
    }

    pub fn write(&self, stream: &mut impl Write) -> Result<()> {
        self.status.write_status_line(stream)?;

        // Set Content-Length: 0 for empty body responses
        if matches!(self.body, HttpBody::Empty) {
            let mut headers = self.headers.clone();
            headers.set_content_length(0);
            headers.write(stream)?;
        } else {
            self.headers.write(stream)?;
        }

        // empty line to separate body from headers
        stream.write_all(CRLF)?;
        if let HttpBody::Content(body) = &self.body {
            stream.write_all(body.as_slice())?;
        }

        stream.flush()?;
        Ok(())
    }
}

pub fn bad_request(body: &str) -> Response {
    let mut resp = Response::new(HttpStatus::BadRequest);
    resp.set_str_body(body);
    resp
}

pub fn ok() -> Response {
    Response::new(HttpStatus::Ok)
}

pub fn not_found() -> Response {
    Response::new(HttpStatus::NotFound)
}

pub fn internal_server_error(message: Option<&str>) -> Response {
    let mut resp = Response::new(HttpStatus::InternalServerError);
    if let Some(msg) = message {
        resp.set_str_body(msg);
    }

    resp
}
