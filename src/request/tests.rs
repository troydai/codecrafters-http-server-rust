use std::io::Cursor;

use super::{from_line_stream, from_reader};
use crate::body::HttpBody;
use crate::connection::LineStream;

#[test]
fn test_from_reader_simple_get() {
    let raw_request = b"GET / HTTP/1.1\r\nContent-Type: application/json\r\n\r\n";
    let mut reader = Cursor::new(raw_request.as_slice());

    let request = from_reader(&mut reader).expect("should parse request");

    assert_eq!(request.path(), "/");
}

#[test]
fn test_from_reader_with_empty_body() {
    let raw_request = b"POST /submit HTTP/1.1\r\nContent-Length: 0\r\n\r\n";
    let mut reader = Cursor::new(raw_request.as_slice());

    let request = from_reader(&mut reader).expect("should parse request");

    assert_eq!(request.path(), "/submit");
    assert!(matches!(request.body(), HttpBody::Empty));
}

#[test]
fn test_from_reader_with_body() {
    let body = b"Hello, World!";
    let raw_request = format!(
        "POST /submit HTTP/1.1\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        std::str::from_utf8(body).unwrap()
    );
    let mut reader = Cursor::new(raw_request.as_bytes());

    let request = from_reader(&mut reader).expect("should parse request");

    assert_eq!(request.path(), "/submit");
    match request.body() {
        HttpBody::Content(data) => assert_eq!(data, body),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }
}

#[test]
fn test_from_reader_with_json_body() {
    let body = br#"{"name": "test", "value": 123}"#;
    let raw_request = format!(
        "POST /api/data HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        std::str::from_utf8(body).unwrap()
    );
    let mut reader = Cursor::new(raw_request.as_bytes());

    let request = from_reader(&mut reader).expect("should parse request");

    assert_eq!(request.path(), "/api/data");
    match request.body() {
        HttpBody::Content(data) => assert_eq!(data, body),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }
}

#[test]
fn test_from_reader_with_large_body() {
    // Body larger than 1024 bytes to test chunked reading
    #[allow(clippy::cast_possible_truncation)]
    let body: Vec<u8> = (0_u32..2500).map(|i| (i % 256) as u8).collect();
    let body_str = String::from_utf8_lossy(&body);
    let raw_request = format!(
        "POST /upload HTTP/1.1\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body_str
    );
    let mut reader = Cursor::new(raw_request.into_bytes());

    let request = from_reader(&mut reader).expect("should parse request");

    assert_eq!(request.path(), "/upload");
    match request.body() {
        HttpBody::Content(data) => assert_eq!(data.len(), 2500),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }
}

#[test]
fn test_from_reader_no_content_length_header() {
    let raw_request = b"GET /path HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let mut reader = Cursor::new(raw_request.as_slice());

    let request = from_reader(&mut reader).expect("should parse request");

    assert_eq!(request.path(), "/path");
    assert!(matches!(request.body(), HttpBody::Empty));
}

#[test]
fn test_from_reader_body_with_binary_data() {
    let body: Vec<u8> = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD];
    let mut raw_request = format!(
        "POST /binary HTTP/1.1\r\nContent-Length: {}\r\n\r\n",
        body.len()
    )
    .into_bytes();
    raw_request.extend(&body);
    let mut reader = Cursor::new(raw_request);

    let request = from_reader(&mut reader).expect("should parse request");

    assert_eq!(request.path(), "/binary");
    match request.body() {
        HttpBody::Content(data) => assert_eq!(data, &body),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }
}

#[test]
fn test_from_reader_simple_get_long_path() {
    let raw_request = b"GET /aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa HTTP/1.1\r\n\r\n";
    let mut reader = Cursor::new(raw_request.as_slice());

    let request = from_reader(&mut reader).expect("should parse request");

    assert_eq!(
        request.path(),
        "/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
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
fn test_from_reader_split_crlf() {
    // \r is at the end of first chunk, \n is at the start of second chunk
    let chunks = vec![b"GET / HTTP/1.1\r".to_vec(), b"\n\r\n".to_vec()];
    let mut reader = MockReader::new(chunks);

    let request = from_reader(&mut reader).expect("should parse request");
    assert_eq!(request.path(), "/");
}

#[test]
fn test_from_line_stream_pipelined_requests() {
    // Two pipelined GET requests in a single buffer
    let raw_requests = b"GET /first HTTP/1.1\r\nHost: example.com\r\n\r\nGET /second HTTP/1.1\r\nHost: example.com\r\n\r\n";
    let mut reader = Cursor::new(raw_requests.as_slice());
    let mut ls = LineStream::new(&mut reader);

    // Parse first request
    let req1 = from_line_stream(&mut ls).expect("should parse first request");
    assert_eq!(req1.path(), "/first");

    // Parse second request from the same LineStream
    let req2 = from_line_stream(&mut ls).expect("should parse second request");
    assert_eq!(req2.path(), "/second");
}

#[test]
fn test_from_line_stream_pipelined_with_body() {
    // Pipelined POST request with body followed by GET request
    let body1 = b"Hello, World!";
    let raw_requests = format!(
        "POST /submit HTTP/1.1\r\nContent-Length: {}\r\n\r\n{}GET /status HTTP/1.1\r\n\r\n",
        body1.len(),
        std::str::from_utf8(body1).unwrap()
    );
    let mut reader = Cursor::new(raw_requests.as_bytes());
    let mut ls = LineStream::new(&mut reader);

    // Parse first request (POST with body)
    let req1 = from_line_stream(&mut ls).expect("should parse first request");
    assert_eq!(req1.path(), "/submit");
    match req1.body() {
        HttpBody::Content(data) => assert_eq!(data, body1),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }

    // Parse second request (GET)
    let req2 = from_line_stream(&mut ls).expect("should parse second request");
    assert_eq!(req2.path(), "/status");
    assert!(matches!(req2.body(), HttpBody::Empty));
}

#[test]
fn test_from_line_stream_multiple_pipelined_requests() {
    // Three pipelined requests
    let raw_requests =
        b"GET /one HTTP/1.1\r\n\r\nGET /two HTTP/1.1\r\n\r\nGET /three HTTP/1.1\r\n\r\n";
    let mut reader = Cursor::new(raw_requests.as_slice());
    let mut ls = LineStream::new(&mut reader);

    let req1 = from_line_stream(&mut ls).expect("should parse first request");
    assert_eq!(req1.path(), "/one");

    let req2 = from_line_stream(&mut ls).expect("should parse second request");
    assert_eq!(req2.path(), "/two");

    let req3 = from_line_stream(&mut ls).expect("should parse third request");
    assert_eq!(req3.path(), "/three");
}

#[test]
fn test_from_line_stream_pipelined_with_large_body() {
    // Pipelined requests where one has a large body (> 1024 bytes)
    // Use ASCII printable characters to avoid UTF-8 issues
    let body: Vec<u8> = (0_u8..255).cycle().take(1500).collect(); // ASCII printable range

    // Build the request manually to ensure exact byte count
    let mut raw_requests = Vec::new();
    raw_requests.extend_from_slice(b"POST /upload HTTP/1.1\r\nContent-Length: ");
    raw_requests.extend_from_slice(body.len().to_string().as_bytes());
    raw_requests.extend_from_slice(b"\r\n\r\n");
    raw_requests.extend_from_slice(&body);
    raw_requests.extend_from_slice(b"GET /done HTTP/1.1\r\n\r\n");

    let mut reader = Cursor::new(raw_requests);
    let mut ls = LineStream::new(&mut reader);

    // Parse first request with large body
    let req1 = from_line_stream(&mut ls).expect("should parse first request");
    assert_eq!(req1.path(), "/upload");
    match req1.body() {
        HttpBody::Content(data) => {
            assert_eq!(data.len(), 1500);
            assert_eq!(data, &body);
        }
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }

    // Parse second request
    let req2 = from_line_stream(&mut ls).expect("should parse second request");
    assert_eq!(req2.path(), "/done");
}

#[test]
fn test_from_line_stream_pipelined_mixed_methods() {
    // Mix of different HTTP methods pipelined together
    let body = b"data";
    let raw_requests = format!(
        "GET /resource HTTP/1.1\r\n\r\nPOST /resource HTTP/1.1\r\nContent-Length: {}\r\n\r\n{}DELETE /resource HTTP/1.1\r\n\r\n",
        body.len(),
        std::str::from_utf8(body).unwrap()
    );
    let mut reader = Cursor::new(raw_requests.as_bytes());
    let mut ls = LineStream::new(&mut reader);

    let req1 = from_line_stream(&mut ls).expect("should parse GET request");
    assert_eq!(req1.path(), "/resource");

    let req2 = from_line_stream(&mut ls).expect("should parse POST request");
    assert_eq!(req2.path(), "/resource");
    match req2.body() {
        HttpBody::Content(data) => assert_eq!(data, body),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }

    let req3 = from_line_stream(&mut ls).expect("should parse DELETE request");
    assert_eq!(req3.path(), "/resource");
}
