#[cfg(test)]
mod tests;

use anyhow::{Result, anyhow};
use std::io::Read;

use crate::body::HttpBody;
use crate::consts::{self, CRLF};
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

pub fn from_reader(reader: &mut impl Read) -> Result<Request> {
    let mut req: Option<Request> = None;
    let mut current_line: Vec<u8> = Vec::new();
    let mut left_over: Vec<u8> = Vec::new();

    'parent: loop {
        let mut buf = [0; 1024];
        let size = reader.read(&mut buf)?;

        let mut start: usize = 0;
        loop {
            if let Some(end) = buf[start..size]
                .windows(2)
                .position(|w| w == CRLF)
                .map(|p| p + start)
            {
                // found a CRLF from starting position, append it to the current line
                // the current line may ore may not be empty.
                current_line.extend_from_slice(&buf[start..end]);

                // end of the headers, push the remaining bytes in the buffer to the left_over
                // in case they're part of the request body
                if current_line.is_empty() {
                    left_over.extend_from_slice(&buf[end + 2..size]);
                    break 'parent;
                }

                match req {
                    None => req = Some(Request::from_request_line(&current_line)?),
                    Some(ref mut r) => {
                        r.headers.read(&current_line)?;
                    }
                }

                current_line.clear();

                start = end + 2;
            } else {
                // CRLF is not found. remembers the remaining bytes
                // and read another batch of bytes from the stream
                current_line.extend(buf[start..size].iter());
                break;
            }
        }
    }

    req.map_or_else(
        || Err(anyhow!("unable to parse the request")),
        |mut req| {
            let content_length = req.headers.content_length()?;
            req.body = HttpBody::read(Some(&left_over), content_length, reader)?;
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
