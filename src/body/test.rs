use super::*;
use std::io::Cursor;

#[test]
fn test_content_length_condition() {}

#[test]
fn test_read_empty_body() {
    let mut stream = Cursor::new(Vec::new());
    let result = HttpBody::read(None, 0, &mut stream).unwrap();
    assert!(matches!(result, HttpBody::Empty));
}

#[test]
fn test_read_body_from_stream_only() {
    let content = b"Hello, World!";
    let mut stream = Cursor::new(content.to_vec());
    let result = HttpBody::read(None, content.len(), &mut stream).unwrap();

    match result {
        HttpBody::Content(data) => assert_eq!(data, content.to_vec()),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }
}

#[test]
fn test_read_body_with_preread_data() {
    let preread = b"Hello, ";
    let remaining = b"World!";
    let mut stream = Cursor::new(remaining.to_vec());
    let total_len = preread.len() + remaining.len();

    let result = HttpBody::read(Some(preread), total_len, &mut stream).unwrap();

    match result {
        HttpBody::Content(data) => assert_eq!(data, b"Hello, World!".to_vec()),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }
}

#[test]
fn test_read_body_all_preread() {
    let preread = b"All preread data";
    let mut stream = Cursor::new(Vec::new());

    let result = HttpBody::read(Some(preread), preread.len(), &mut stream).unwrap();

    match result {
        HttpBody::Content(data) => assert_eq!(data, preread.to_vec()),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }
}

#[test]
fn test_read_large_body_multiple_chunks() {
    // Create content larger than 1024 bytes to test chunked reading
    let content: Vec<u8> = (0..2500).map(|i| (i % 256) as u8).collect();
    let mut stream = Cursor::new(content.clone());

    let result = HttpBody::read(None, content.len(), &mut stream).unwrap();

    match result {
        HttpBody::Content(data) => assert_eq!(data, content),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }
}

#[test]
fn test_read_exactly_1024_bytes() {
    let content: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    let mut stream = Cursor::new(content.clone());

    let result = HttpBody::read(None, content.len(), &mut stream).unwrap();

    match result {
        HttpBody::Content(data) => assert_eq!(data, content),
        HttpBody::Empty => panic!("Expected Content, got Empty"),
    }
}

#[test]
fn test_read_stream_error() {
    // A reader that always returns an error
    struct ErrorReader;
    impl std::io::Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "test error"))
        }
    }

    let mut stream = ErrorReader;
    let result = HttpBody::read(None, 10, &mut stream);

    assert!(result.is_err());
}
