#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::consts;
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Headers {
    headers: HashMap<String, Vec<String>>,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, value: &str) {
        let _ = self
            .headers
            .entry(name.to_lowercase())
            .and_modify(|coll| coll.push(String::from(value)))
            .or_insert_with(|| Vec::from([String::from(value)]));
    }

    /// get returns the value of the header that matches the name. if
    /// there are multiple values, the first one is returned. returns
    /// None if the name doesn't match any headers
    pub fn get(&self, name: &str) -> Option<&str> {
        if let Some(values) = self.headers.get(&name.to_lowercase()) {
            if values.is_empty() {
                // should not be possible
                return None;
            }

            return Some(&values[0]);
        }

        None
    }

    /* For future use

    pub fn delete(&mut self, name: &str) {
        let _ = self.headers.remove(name);
    }

    /// set clears all values associated with the given name and set
    /// its value to the singe value provided.
    pub fn set(&mut self, name: &str, value: &str) {
        let _ = self
            .headers
            .entry(String::from(name))
            .and_modify(|coll| {
                coll.clear();
                coll.push(String::from(value));
            })
            .or_insert(Vec::from([String::from(value)]));
    }

    */

    pub fn write(&self, stream: &mut impl std::io::Write) -> Result<()> {
        self.headers.iter().try_for_each(|(k, v)| -> Result<()> {
            let bytes = wire_format(k, v);
            stream.write_all(&bytes)?;

            Ok(())
        })?;

        Ok(())
    }

    /// read accept a slice of bytes that represent a line of header
    /// in the HTTP request and return the name-value pair. it returns error
    /// if the input is not a valid HTTP header.
    pub fn read(&mut self, bytes: &[u8]) -> Result<()> {
        let s = std::str::from_utf8(bytes)?;
        if let Some(idx) = s.find(':') {
            let name = std::str::from_utf8(&bytes[..idx])?.trim().to_lowercase();
            let value = std::str::from_utf8(&bytes[idx + 1..])?.trim();

            self.add(&name, value);
            return Ok(());
        }

        Err(anyhow!("invalid header bytes"))
    }
}

fn wire_format(name: &String, values: &[String]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.extend_from_slice(name.as_bytes());
    result.extend_from_slice(consts::COLON);
    result.extend_from_slice(consts::SPACE);

    let mut first = true;
    for each in values {
        if !first {
            result.extend_from_slice(consts::COMMA);
            result.extend_from_slice(consts::SPACE);
        }

        result.extend_from_slice(each.as_bytes());
        first = false;
    }

    result.extend_from_slice(consts::CRLF);

    result
}
