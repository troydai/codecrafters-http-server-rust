#[cfg(test)]
mod tests;

use anyhow::{Result, anyhow};
use std::io::Read;

use crate::body::HttpBody;
use crate::connection::LineStream;
use crate::consts;
use crate::header::Headers;
use crate::http::method::HttpMethod;

#[derive(Debug)]
pub struct Request {
    #[allow(dead_code)]
    method: HttpMethod,
    path: String,
    headers: Headers,
    body: HttpBody,
}

/// Parses an HTTP request from a `LineStream`.
///
/// This function preserves the internal buffer state of the `LineStream`,
/// making it suitable for parsing multiple pipelined requests from a single
/// TCP connection (HTTP/1.1 keep-alive).
///
/// # Arguments
///
/// * `ls` - A mutable reference to a `LineStream` that wraps the underlying reader
///
/// # Returns
///
/// * `Ok(Request)` - Successfully parsed HTTP request
/// * `Err(_)` - If the request is malformed or an I/O error occurs
pub fn from_line_stream<T: Read>(ls: &mut LineStream<T>) -> Result<Request> {
    let mut req: Option<Request> = None;

    loop {
        let current_line = ls.read_line()?;

        // end of the headers, push the remaining bytes in the buffer to the left_over
        // in case they're part of the request body
        if current_line.is_empty() {
            break;
        }

        match req {
            None => req = Some(Request::from_request_line(&current_line)?),
            Some(ref mut r) => {
                r.headers.read(&current_line)?;
            }
        }
    }

    req.map_or_else(
        || Err(anyhow!("unable to parse the request")),
        |mut req| {
            let content_length = req.headers.content_length()?;
            if content_length != 0 {
                let body = ls.read_bytes(content_length)?;
                req.body = HttpBody::Content(body);
            }

            Ok(req)
        },
    )
}

impl Request {
    fn from_request_line(bytes: &[u8]) -> Result<Self> {
        let rl = RequestLine::from_bytes(bytes)?;
        let method = rl.method.parse()?;
        let path = String::from(rl.path);

        Ok(Self {
            method,
            path,
            headers: Headers::new(),
            body: HttpBody::Empty,
        })
    }

    pub fn path_match_exact(&self, pattern: &str) -> bool {
        pattern == self.path
    }

    pub fn path_match_prefix(&self, pattern: &str) -> bool {
        self.path.starts_with(pattern)
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub const fn headers(&self) -> &Headers {
        &self.headers
    }

    pub const fn body(&self) -> &HttpBody {
        &self.body
    }
}

struct RequestLine<'a> {
    method: &'a str,
    path: &'a str,
}

impl<'a> RequestLine<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        let parts = std::str::from_utf8(bytes)?
            .split(' ')
            .collect::<Vec<&str>>();

        if parts.len() < 3 {
            return Err(anyhow::anyhow!(
                "request's first line is malformed. fewer than 3 parts"
            ));
        }

        if parts[2] != consts::STR_HTTP_1_1 {
            return Err(anyhow::anyhow!("protocol {} is not supported", parts[2]));
        }

        Ok(RequestLine {
            method: parts[0],
            path: parts[1],
        })
    }
}
