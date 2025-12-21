#[cfg(test)]
mod tests;

use anyhow::{Result, bail};
use std::io::Read;

use crate::consts::CRLF;
use crate::header::Headers;

#[derive(Debug)]
pub struct Request {
    _protocol: String,
    _method: String,
    headers: Headers,
    path: String,
}

pub fn from_reader(reader: &mut impl Read) -> Result<Request> {
    let mut headers: Vec<Vec<u8>> = Vec::new();
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

                headers.push(current_line.clone());
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

    Request::from_header(headers)
}

impl Request {
    pub fn from_header(headers: Vec<Vec<u8>>) -> Result<Self> {
        if headers.is_empty() {
            bail!("invalid headers number")
        }

        // extract method, path, and protocol from first line
        let first_line_bytes = headers[0].clone();
        let first_line_str = std::str::from_utf8(&first_line_bytes)?;
        let parts: Vec<&str> = first_line_str.split(' ').collect();
        if parts.len() < 3 {
            bail!("request's first line is malformed. fewer than 3 parts")
        }

        let headers = Headers::from_bytes(&headers.into_iter().skip(1).collect::<Vec<_>>())?;

        Ok(Self {
            _method: String::from(parts[0]).to_uppercase(),
            headers,
            path: String::from(parts[1]),
            _protocol: String::from(parts[2]),
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn headers(&self) -> &Headers {
        &self.headers
    }
}
