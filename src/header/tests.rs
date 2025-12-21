use super::{Header, Headers};

#[test]
fn test_header_new_stores_name_lowercase() {
    let header = Header::new("Content-Type", "application/json");
    assert_eq!(header.name, "content-type");
    assert_eq!(header.first_value(), "application/json");
}

#[test]
fn test_header_new_preserves_already_lowercase() {
    let header = Header::new("accept", "text/html");
    assert_eq!(header.name, "accept");
}

#[test]
fn test_header_from_str_stores_name_lowercase() {
    let header = Header::from_str("Content-Type: application/json").unwrap();
    assert_eq!(header.name, "content-type");
    assert_eq!(header.first_value(), "application/json");
}

#[test]
fn test_header_from_str_mixed_case() {
    let header = Header::from_str("X-Custom-Header: some-value").unwrap();
    assert_eq!(header.name, "x-custom-header");
}

#[test]
fn test_headers_value_case_insensitive_lookup() {
    let mut headers = Headers::new();
    headers.add_header(Header::new("Content-Type", "application/json"));

    // Should find with exact lowercase
    assert!(headers.value("content-type").is_some());

    // Should find with original case
    assert!(headers.value("Content-Type").is_some());

    // Should find with uppercase
    assert!(headers.value("CONTENT-TYPE").is_some());

    // Should find with mixed case
    assert!(headers.value("CoNtEnT-TyPe").is_some());
}

#[test]
fn test_headers_value_returns_correct_header() {
    let mut headers = Headers::new();
    headers.add_header(Header::new("Accept", "text/html"));
    headers.add_header(Header::new("Content-Type", "application/json"));

    let header = headers.value("CONTENT-TYPE").unwrap();
    assert_eq!(header.first_value(), "application/json");
}

#[test]
fn test_headers_value_not_found() {
    let mut headers = Headers::new();
    headers.add_header(Header::new("Content-Type", "application/json"));

    assert!(headers.value("Accept").is_none());
}

#[test]
fn test_headers_from_bytes_case_insensitive() {
    let data: Vec<Vec<u8>> = vec![
        b"Content-Type: application/json".to_vec(),
        b"X-REQUEST-ID: 12345".to_vec(),
    ];

    let headers = Headers::from_bytes(&data).unwrap();

    // Both should be findable with any case
    assert!(headers.value("content-type").is_some());
    assert!(headers.value("CONTENT-TYPE").is_some());
    assert!(headers.value("x-request-id").is_some());
    assert!(headers.value("X-Request-Id").is_some());
}
