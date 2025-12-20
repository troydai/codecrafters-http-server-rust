use anyhow::{Result, bail};
use std::io::Read;
use std::net::TcpStream;

use crate::consts::CRLF;

#[derive(Debug)]
pub struct Request {
    _method: String,
    pub path: String,
    pub protocol: String,
}

pub fn from_stream(stream: &mut TcpStream) -> Result<Request> {
    let mut line: Vec<u8> = Vec::new();
    loop {
        let mut buf = [0; 1024];
        let size = stream.read(&mut buf)?;

        if let Some(pos) = buf[..size].windows(2).position(|w| w == CRLF) {
            line.extend(&buf[..pos]);
            break;
        } else {
            line.extend(&buf);
        }
    }

    Request::from_header(std::str::from_utf8(&line[..])?)
}

impl Request {
    pub fn _new() -> Self {
        Self {
            _method: String::from("GET"),
            path: String::from("/"),
            protocol: String::from("/"),
        }
    }

    pub fn from_header(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() < 3 {
            bail!("request's first line is malformed. fewer than 3 parts")
        }

        Ok(Self {
            _method: String::from(parts[0]).to_uppercase(),
            path: String::from(parts[1]),
            protocol: String::from(parts[2]),
        })
    }
}
