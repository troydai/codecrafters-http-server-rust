use anyhow::{Result, bail};

#[derive(Debug)]
pub struct Request {
    _method: String,
    pub path: String,
    pub protocol: String,
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
