use crate::consts;
use anyhow::Result;

#[derive(Debug)]
pub struct Headers {
    headers: Vec<Header>,
}

impl Headers {
    pub const fn new() -> Self {
        Self {
            headers: Vec::new(),
        }
    }

    pub fn from_bytes(data: &[Vec<u8>]) -> Result<Self> {
        let mut retval = Self::new();

        for line in data {
            let s = std::str::from_utf8(line)?;
            let h = Header::from_str(s)?;
            retval.headers.push(h);
        }

        Ok(retval)
    }

    pub fn add_header(&mut self, h: Header) {
        self.headers.push(h);
    }

    pub fn vec(&self) -> Vec<&Header> {
        self.headers.iter().collect::<Vec<_>>()
    }
}

#[derive(Debug)]
pub struct Header {
    name: String,
    values: Vec<String>,
}

impl Header {
    pub fn new(name: &str, value: &str) -> Self {
        let values: Vec<String> = Vec::from([String::from(value)]);
        Self {
            name: String::from(name),
            values,
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        if let Some(idx) = s.find(':') {
            let name = String::from(&s[0..idx]);
            let values: Vec<String> = Vec::from([String::from(&s[idx + 1..])]);

            return Ok(Self { name, values });
        }

        Err(anyhow::anyhow!("Invalid header format"))
    }

    pub fn wire_representation(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(self.name.as_bytes());
        result.extend_from_slice(consts::COLON);
        result.extend_from_slice(consts::SPACE);

        let mut first = true;
        self.values.iter().for_each(|h| {
            if !first {
                result.extend_from_slice(consts::COMMA);
                result.extend_from_slice(consts::SPACE);
            }

            result.extend_from_slice(h.as_bytes());
            first = false;
        });

        result.extend_from_slice(consts::CRLF);

        result
    }
}
