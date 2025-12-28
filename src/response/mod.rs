#[cfg(test)]
mod tests;

use crate::consts::CRLF;
use crate::header::Headers;
use crate::http::status::HttpStatus;
use anyhow::Result;
use std::io::Write;

#[derive(Debug)]
pub struct Response {
    status: HttpStatus,
    headers: Headers,
    body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(status: HttpStatus) -> Self {
        Self {
            status,
            headers: Headers::new(),
            body: None,
        }
    }

    pub fn set_str_body(&mut self, body: &str) {
        let bytes = body.as_bytes();
        self.headers.set("Content-Type", "text/plain");
        self.headers.set("Content-Length", &bytes.len().to_string());
        self.body = Some(Vec::from(body.as_bytes()));
    }

    pub fn set_bytes_body(&mut self, content_type: &str, body: &[u8]) {
        self.headers.set("Content-Type", content_type);
        self.headers.set("Content-Length", &body.len().to_string());
        self.body = Some(Vec::from(body));
    }

    pub fn write(&self, stream: &mut impl Write) -> Result<()> {
        self.status.write_status_line(stream)?;

        // For HTTP/1.1 persistent connections, responses without a body must
        // include Content-Length: 0 so clients know the response is complete.
        // We clone headers to add Content-Length: 0 when body is absent,
        // ensuring all headers go through the Headers struct consistently.
        if self.body.is_none() {
            let mut headers = self.headers.clone();
            headers.set_content_length(0);
            headers.write(stream)?;
        } else {
            self.headers.write(stream)?;
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
