use super::*;
use std::io::Cursor;

#[test]
fn test_read_single_line() {
    let data = b"Hello, World!\r\n";
    let mut stream = Cursor::new(data.to_vec());
    let mut line_stream = LineStream::new(&mut stream);

    let result = line_stream.read_line().unwrap();
    assert_eq!(result, b"Hello, World!".to_vec());
}

#[test]
fn test_read_multiple_lines() {
    let data = b"Line 1\r\nLine 2\r\nLine 3\r\n";
    let mut stream = Cursor::new(data.to_vec());
    let mut line_stream = LineStream::new(&mut stream);

    assert_eq!(line_stream.read_line().unwrap(), b"Line 1".to_vec());
    assert_eq!(line_stream.read_line().unwrap(), b"Line 2".to_vec());
    assert_eq!(line_stream.read_line().unwrap(), b"Line 3".to_vec());
}

#[test]
fn test_read_empty_line() {
    let data = b"\r\n";
    let mut stream = Cursor::new(data.to_vec());
    let mut line_stream = LineStream::new(&mut stream);

    let result = line_stream.read_line().unwrap();
    assert_eq!(result, b"".to_vec());
}

#[test]
fn test_read_line_with_special_characters() {
    let data = b"Content-Type: text/html\r\n";
    let mut stream = Cursor::new(data.to_vec());
    let mut line_stream = LineStream::new(&mut stream);

    let result = line_stream.read_line().unwrap();
    assert_eq!(result, b"Content-Type: text/html".to_vec());
}

#[test]
fn test_read_line_larger_than_buffer() {
    // Create a line larger than the 1024 byte buffer
    let mut large_content: Vec<u8> = (0_u32..2000).map(|i| b'a' + (i % 26) as u8).collect();
    large_content.extend_from_slice(b"\r\n");

    let mut stream = Cursor::new(large_content.clone());
    let mut line_stream = LineStream::new(&mut stream);

    let result = line_stream.read_line().unwrap();
    // Result should be the content without the CRLF
    assert_eq!(result.len(), 2000);
    assert_eq!(result, large_content[..2000].to_vec());
}

#[test]
fn test_read_http_request_line() {
    let data = b"GET /index.html HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let mut stream = Cursor::new(data.to_vec());
    let mut line_stream = LineStream::new(&mut stream);

    assert_eq!(
        line_stream.read_line().unwrap(),
        b"GET /index.html HTTP/1.1".to_vec()
    );
    assert_eq!(
        line_stream.read_line().unwrap(),
        b"Host: localhost".to_vec()
    );
    assert_eq!(line_stream.read_line().unwrap(), b"".to_vec());
}

#[test]
fn test_read_line_with_only_cr() {
    // A line with only CR should not be treated as a line terminator
    let data = b"Hello\rWorld\r\n";
    let mut stream = Cursor::new(data.to_vec());
    let mut line_stream = LineStream::new(&mut stream);

    let result = line_stream.read_line().unwrap();
    assert_eq!(result, b"Hello\rWorld".to_vec());
}

#[test]
fn test_read_line_with_only_lf() {
    // A line with only LF should not be treated as a line terminator
    let data = b"Hello\nWorld\r\n";
    let mut stream = Cursor::new(data.to_vec());
    let mut line_stream = LineStream::new(&mut stream);

    let result = line_stream.read_line().unwrap();
    assert_eq!(result, b"Hello\nWorld".to_vec());
}

#[test]
fn test_read_binary_data() {
    let mut data: Vec<u8> = vec![0x00, 0x01, 0x02, 0xFF, 0xFE];
    data.extend_from_slice(b"\r\n");

    let mut stream = Cursor::new(data.clone());
    let mut line_stream = LineStream::new(&mut stream);

    let result = line_stream.read_line().unwrap();
    assert_eq!(result, vec![0x00, 0x01, 0x02, 0xFF, 0xFE]);
}

#[test]
fn test_read_consecutive_crlf() {
    let data = b"\r\n\r\n\r\n";
    let mut stream = Cursor::new(data.to_vec());
    let mut line_stream = LineStream::new(&mut stream);

    assert_eq!(line_stream.read_line().unwrap(), b"".to_vec());
    assert_eq!(line_stream.read_line().unwrap(), b"".to_vec());
    assert_eq!(line_stream.read_line().unwrap(), b"".to_vec());
}

#[test]
// #[timeout(1000)] // Prevent infinite loop from hanging the test runner forever
fn test_read_eof_no_crlf() {
    let data = b"Hello";
    let mut stream = Cursor::new(data.to_vec());
    let mut line_stream = LineStream::new(&mut stream);

    // This is expected to either return the partial line or error, but NOT hang.
    // Based on current implementation, it might hang.
    let result = line_stream.read_line();
    assert!(result.is_err() || result.unwrap() == b"Hello");
}

struct MockReader {
    chunks: Vec<Vec<u8>>,
    current_chunk: usize,
    position_in_chunk: usize,
}

impl MockReader {
    fn new(chunks: Vec<Vec<u8>>) -> Self {
        Self {
            chunks,
            current_chunk: 0,
            position_in_chunk: 0,
        }
    }
}

impl std::io::Read for MockReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.current_chunk >= self.chunks.len() {
            return Ok(0);
        }

        let chunk = &self.chunks[self.current_chunk];
        let remaining = chunk.len() - self.position_in_chunk;

        let to_copy = std::cmp::min(remaining, buf.len());
        buf[..to_copy]
            .copy_from_slice(&chunk[self.position_in_chunk..self.position_in_chunk + to_copy]);

        self.position_in_chunk += to_copy;
        if self.position_in_chunk >= chunk.len() {
            self.current_chunk += 1;
            self.position_in_chunk = 0;
        }

        Ok(to_copy)
    }
}

#[test]
fn test_read_split_crlf() {
    // \r is at the end of first chunk, \n is at the start of second chunk
    let chunks = vec![b"Hello\r".to_vec(), b"\nWorld\r\n".to_vec()];
    let mut stream = MockReader::new(chunks);
    let mut line_stream = LineStream::new(&mut stream);

    let result = line_stream.read_line().unwrap();
    assert_eq!(result, b"Hello".to_vec());

    let result = line_stream.read_line().unwrap();
    assert_eq!(result, b"World".to_vec());
}

#[test]
fn test_read_bytes_larger_than_buffer() {
    let large_content: Vec<u8> = (0_u32..2000).map(|i| (i % 255) as u8).collect();
    let mut stream = Cursor::new(large_content.clone());
    let mut line_stream = LineStream::new(&mut stream);

    let result = line_stream.read_bytes(2000).unwrap();
    assert_eq!(result, large_content);
}

#[test]
fn test_read_bytes_exact_buffer_size() {
    let content: Vec<u8> = (0_u32..1024).map(|i| (i % 255) as u8).collect();
    let mut stream = Cursor::new(content.clone());
    let mut line_stream = LineStream::new(&mut stream);

    // This forces `read_bytes` to read exactly 1024 bytes.
    // If we assume default buffer is 1024, this hits boundary conditions.
    let result = line_stream.read_bytes(1024).unwrap();
    assert_eq!(result, content);
}

#[test]
fn test_read_bytes_split_buffers() {
    // Scenario:
    // 1. We read a line, which leaves some data in `stream_buffer`.
    // 2. We then read bytes. Some should come from `stream_buffer`, some from `stream`.

    // "Line\r\n" is 6 bytes.
    // "Body..." will follow.
    let line = b"Line\r\n";
    let body_part1 = b"BodyPart1"; // 9 bytes
    let body_part2 = b"BodyPart2"; // 9 bytes

    // We want `read_line` to pull everything into its internal buffer (if small enough)
    // or at least pull `body_part1` into `stream_buffer`.
    // To ensure `body_part1` is in `stream_buffer` but `body_part2` is NOT (to test reading from stream),
    // we need to construct a MockReader that delivers chunks.

    let chunks = vec![
        // Chunk 1: Line + BodyPart1.
        // read_line will consume Line. BodyPart1 stays in stream_buffer.
        {
            let mut c = Vec::new();
            c.extend_from_slice(line);
            c.extend_from_slice(body_part1);
            c
        },
        // Chunk 2: BodyPart2.
        // read_bytes should fetch this from stream after draining stream_buffer.
        body_part2.to_vec(),
    ];

    let mut stream = MockReader::new(chunks);
    let mut line_stream = LineStream::new(&mut stream);

    // 1. Read Line
    let read_line = line_stream.read_line().unwrap();
    assert_eq!(read_line, b"Line".to_vec());

    // 2. Read Body (Part1 + Part2)
    let total_len = body_part1.len() + body_part2.len();
    let read_body = line_stream.read_bytes(total_len).unwrap();

    let mut expected_body = Vec::new();
    expected_body.extend_from_slice(body_part1);
    expected_body.extend_from_slice(body_part2);

    assert_eq!(read_body, expected_body);
}

#[test]
fn test_consecutive_requests() {
    let req1 = b"GET /first HTTP/1.1\r\nContent-Length: 5\r\n\r\nHello";
    let req2 = b"GET /second HTTP/1.1\r\nContent-Length: 6\r\n\r\nWorld!";

    let mut data = Vec::new();
    data.extend_from_slice(req1);
    data.extend_from_slice(req2);

    let mut stream = Cursor::new(data);
    let mut line_stream = LineStream::new(&mut stream);

    // Parse Request 1 manually using LineStream
    let line1 = line_stream.read_line().unwrap();
    assert_eq!(line1, b"GET /first HTTP/1.1");

    let line2 = line_stream.read_line().unwrap();
    assert_eq!(line2, b"Content-Length: 5");

    let line3 = line_stream.read_line().unwrap();
    assert_eq!(line3, b""); // Empty line after headers

    let body1 = line_stream.read_bytes(5).unwrap();
    assert_eq!(body1, b"Hello");

    // Parse Request 2
    let line1_2 = line_stream.read_line().unwrap();
    assert_eq!(line1_2, b"GET /second HTTP/1.1");

    let line2_2 = line_stream.read_line().unwrap();
    assert_eq!(line2_2, b"Content-Length: 6");

    let line3_2 = line_stream.read_line().unwrap();
    assert_eq!(line3_2, b"");

    let body2 = line_stream.read_bytes(6).unwrap();
    assert_eq!(body2, b"World!");
}
