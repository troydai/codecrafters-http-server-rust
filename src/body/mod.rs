use anyhow::Result;

#[derive(Debug)]
pub enum HttpBody {
    Empty,
    #[allow(dead_code)]
    Content(Vec<u8>),
}

impl HttpBody {
    /// `new` creates an `HttpBody`
    pub fn read<F>(read: Option<&[u8]>, content_len: usize, stream: &mut F) -> Result<Self>
    where
        F: std::io::Read,
    {
        // allocate a vector for return data
        let mut data: Vec<u8> = read.map_or_else(
            || Vec::with_capacity(content_len),
            |data| {
                let mut retval = Vec::with_capacity(content_len);
                retval.extend_from_slice(data);
                retval
            },
        );

        let mut remaining = content_len - data.len();
        let mut buf = [0; 1024];
        while remaining > 0 {
            // the amount of data to read is capped at 1024
            let read_len: usize = if remaining < 1024 { remaining } else { 1024 };
            stream.read_exact(&mut buf[0..read_len])?;

            remaining -= read_len;
            data.extend_from_slice(&buf[0..read_len]);
        }

        if data.is_empty() {
            return Ok(Self::Empty);
        }

        Ok(Self::Content(data))
    }
}

#[cfg(test)]
mod test;
