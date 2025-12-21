#[cfg(test)]
mod tests;

use anyhow::{Result, anyhow};
use std::io::Read;

use crate::consts::{self, CRLF};
use crate::header::Headers;
use crate::http::method::HttpMethod;

#[derive(Debug)]
pub struct Request {
    method: Option<HttpMethod>,
    path: Option<String>,
    headers: Headers,
}

pub fn from_reader(reader: &mut impl Read) -> Result<Request> {
    let mut req = Request {
        headers: Headers::new(),
        method: None,
        path: None,
    };

    let mut current_line: Vec<u8> = Vec::new();

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
                current_line.extend(buf[start..end].iter());

                // end of the headers
                if current_line.is_empty() {
                    break 'parent;
                }

                req.fill(&current_line)?;
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

    Ok(req)
}

impl Request {
    /// fill allows the client to gradually build up a HTTP request line
    /// by line. This method is desgined to be called by a reader of the
    /// `TcpStream`. In each call, a slice of bytes represents a line ends
    /// by CRLF. The first call to this function is expected to give it
    /// the request line. After that each call represent a header.
    fn fill(&mut self, bytes: &[u8]) -> Result<()> {
        if self.path.is_none() {
            // expect this bytes slice represents the request line
            let rl = RequestLine::from_bytes(bytes)?;
            self.method = Some(HttpMethod::from_str(rl.method)?);
            self.path = Some(String::from(rl.path));
        } else {
            // the request is already initialized. each of the remaining calls
            // represent a line of headers
            self.headers.read(bytes)?;
        }

        Ok(())
    }

    pub fn path_match_exact(&self, pattern: &str) -> bool {
        if let Some(p) = self.path.as_ref() {
            return pattern == p;
        }

        false
    }

    pub fn path_match_prefix(&self, pattern: &str) -> bool {
        if let Some(p) = self.path.as_ref() {
            return p.starts_with(pattern);
        }

        false
    }

    pub fn path(&self) -> Result<&str> {
        if let Some(p) = self.path.as_ref() {
            return Ok(p);
        }

        Err(anyhow!("request is not initialized"))
    }

    pub const fn headers(&self) -> &Headers {
        &self.headers
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
