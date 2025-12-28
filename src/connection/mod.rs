#[cfg(test)]
mod tests;

use crate::consts;
use anyhow::Result;

/// A wrapper around `std::io::Read` that yields data in CRLF-terminated chunks.
pub struct LineStream<'a, T>
where
    T: std::io::Read,
{
    stream: &'a mut T,
    seperator: &'static [u8],
    stream_buffer: [u8; 1024],
    stream_buffer_size: usize,
    stream_buffer_start: usize,
    line_buffer: Vec<u8>,
}

impl<'a, T> LineStream<'a, T>
where
    T: std::io::Read,
{
    pub fn new(stream: &'a mut T) -> Self {
        Self {
            stream,
            seperator: consts::CRLF,
            stream_buffer: [0; 1024],
            stream_buffer_size: 0,
            stream_buffer_start: 0,
            line_buffer: Vec::with_capacity(2048),
        }
    }

    /// Reads the next CRLF-terminated line from the underlying stream.
    ///
    /// This method buffers data internally and returns complete lines without
    /// the trailing CRLF separator. It blocks until a complete line is available.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - The line content without the CRLF terminator
    /// * `Err(_)` - If an I/O error occurs while reading from the stream
    pub fn read_line(&mut self) -> Result<Vec<u8>> {
        loop {
            // Refill buffer if exhausted
            if self.stream_buffer_start >= self.stream_buffer_size {
                self.stream_buffer_start = 0;
                let bytes_read = self.stream.read(&mut self.stream_buffer)?;
                self.stream_buffer_size = bytes_read;
                
                if bytes_read == 0 {
                     if self.line_buffer.is_empty() {
                         return Err(anyhow::anyhow!("EOF"));
                     }
                     let retval = self.line_buffer.clone();
                     self.line_buffer.clear();
                     return Ok(retval);
                }
            }

            let start = self.stream_buffer_start;
            let size = self.stream_buffer_size;

            if self.stream_buffer_start == 0
                && cross_boundary_separator(&self.line_buffer, &self.stream_buffer)
            {
                // the CR and LR are spread across the bytes that are just read the the bytes
                // that last read. this means the full line is the cached line bytes minus last
                let mut retval = Vec::new();
                retval.extend_from_slice(&self.line_buffer[..self.line_buffer.len() - 1]);
                self.line_buffer.clear();

                // move
                self.stream_buffer_start = 1;

                return Ok(retval);
            }

            if let Some(pos) = self.stream_buffer[start..size]
                .windows(2)
                .position(|w| w == self.seperator)
            {
                let end = start + pos;
                self.line_buffer
                    .extend_from_slice(&self.stream_buffer[start..end]);
                self.stream_buffer_start = end + 2;

                let retval = self.line_buffer.clone();
                self.line_buffer.clear();

                return Ok(retval);
            }

            // No separator found, buffer all remaining data
            self.line_buffer
                .extend_from_slice(&self.stream_buffer[start..size]);
            self.stream_buffer_start = size;
        }
    }

    pub fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut retval = Vec::with_capacity(count);

        // 1. Drain from line_buffer
        let from_line = std::cmp::min(count, self.line_buffer.len());
        retval.extend(self.line_buffer.drain(..from_line));

        if retval.len() == count {
            return Ok(retval);
        }

        // 2. Drain from stream_buffer
        let remaining_in_stream_buf = self.stream_buffer_size - self.stream_buffer_start;
        if remaining_in_stream_buf > 0 {
            let needed = count - retval.len();
            let to_take = std::cmp::min(needed, remaining_in_stream_buf);

            retval.extend_from_slice(&self.stream_buffer[self.stream_buffer_start..self.stream_buffer_start + to_take]);
            self.stream_buffer_start += to_take;

            if retval.len() == count {
                return Ok(retval);
            }
        }

        // 3. Read directly from stream for the rest
        // Extend retval with zeroes to accommodate the read_exact
        let current_len = retval.len();
        retval.resize(count, 0);
        self.stream.read_exact(&mut retval[current_len..])?;

        Ok(retval)
    }

    /// Returns a mutable reference to the underlying stream.
    ///
    /// This allows writing to the stream while preserving the LineStream's
    /// internal buffer state, which is necessary for HTTP keep-alive connections.
    pub fn get_stream_mut(&mut self) -> &mut T {
        self.stream
    }
}

fn cross_boundary_separator(head: &[u8], tail: &[u8]) -> bool {
    if head.is_empty() {
        return false;
    }

    if head[head.len() - 1] != b'\r' {
        return false;
    }

    if tail.is_empty() {
        return false;
    }

    if tail[0] != b'\n' {
        return false;
    }

    true
}
