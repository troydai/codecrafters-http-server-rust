use super::Headers;

#[test]
fn test_new_creates_empty_headers() {
    let headers = Headers::new();
    assert!(headers.get("any-header").is_none());
}

#[test]
fn test_add_stores_name_lowercase() {
    let mut headers = Headers::new();
    headers.add("Content-Type", "application/json");

    // Should be stored lowercase
    assert_eq!(headers.get("content-type"), Some("application/json"));
}

#[test]
fn test_add_preserves_already_lowercase() {
    let mut headers = Headers::new();
    headers.add("accept", "text/html");

    assert_eq!(headers.get("accept"), Some("text/html"));
}

#[test]
fn test_read_stores_name_lowercase() {
    let mut headers = Headers::new();
    headers.read(b"Content-Type: application/json").unwrap();

    assert_eq!(headers.get("content-type"), Some("application/json"));
}

#[test]
fn test_read_mixed_case() {
    let mut headers = Headers::new();
    headers.read(b"X-Custom-Header: some-value").unwrap();

    assert_eq!(headers.get("x-custom-header"), Some("some-value"));
}

#[test]
fn test_get_case_insensitive_lookup() {
    let mut headers = Headers::new();
    headers.add("Content-Type", "application/json");

    // Should find with exact lowercase
    assert!(headers.get("content-type").is_some());

    // Should find with original case
    assert!(headers.get("Content-Type").is_some());

    // Should find with uppercase
    assert!(headers.get("CONTENT-TYPE").is_some());

    // Should find with mixed case
    assert!(headers.get("CoNtEnT-TyPe").is_some());
}

#[test]
fn test_get_returns_correct_value() {
    let mut headers = Headers::new();
    headers.add("Accept", "text/html");
    headers.add("Content-Type", "application/json");

    assert_eq!(headers.get("CONTENT-TYPE"), Some("application/json"));
    assert_eq!(headers.get("accept"), Some("text/html"));
}

#[test]
fn test_get_not_found() {
    let mut headers = Headers::new();
    headers.add("Content-Type", "application/json");

    assert!(headers.get("Accept").is_none());
}

#[test]
fn test_read_case_insensitive() {
    let mut headers = Headers::new();
    headers.read(b"Content-Type: application/json").unwrap();
    headers.read(b"X-REQUEST-ID: 12345").unwrap();

    // Both should be findable with any case
    assert!(headers.get("content-type").is_some());
    assert!(headers.get("CONTENT-TYPE").is_some());
    assert!(headers.get("x-request-id").is_some());
    assert!(headers.get("X-Request-Id").is_some());
}

#[test]
fn test_read_invalid_format() {
    let mut headers = Headers::new();
    let result = headers.read(b"InvalidHeaderWithoutColon");

    assert!(result.is_err());
}

#[test]
fn test_add_multiple_values_same_header() {
    let mut headers = Headers::new();
    headers.add("Accept", "text/html");
    headers.add("Accept", "application/json");

    // get() returns the first value
    assert_eq!(headers.get("accept"), Some("text/html"));
}

// Tests for write() method
#[test]
fn test_write_single_header() {
    let mut headers = Headers::new();
    headers.add("Content-Type", "application/json");

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "content-type: application/json\r\n");
}

#[test]
fn test_write_multiple_headers() {
    let mut headers = Headers::new();
    headers.add("Content-Type", "application/json");
    headers.add("Accept", "text/html");

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();

    // Order may vary due to HashMap, so check both headers are present
    assert!(output.contains("content-type: application/json\r\n"));
    assert!(output.contains("accept: text/html\r\n"));
}

#[test]
fn test_write_multiple_values_same_header() {
    let mut headers = Headers::new();
    headers.add("Accept", "text/html");
    headers.add("Accept", "application/json");
    headers.add("Accept", "application/xml");

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(
        output,
        "accept: text/html, application/json, application/xml\r\n"
    );
}

#[test]
fn test_write_empty_headers() {
    let headers = Headers::new();

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    assert!(buffer.is_empty());
}

// Additional edge case tests for read() method
#[test]
fn test_read_with_spaces_around_name_and_value() {
    let mut headers = Headers::new();
    headers
        .read(b"  Content-Type  :  application/json  ")
        .unwrap();

    assert_eq!(headers.get("content-type"), Some("application/json"));
}

#[test]
fn test_read_with_multiple_colons() {
    let mut headers = Headers::new();
    headers.read(b"X-Custom: value:with:colons").unwrap();

    assert_eq!(headers.get("x-custom"), Some("value:with:colons"));
}

#[test]
fn test_read_with_empty_value() {
    let mut headers = Headers::new();
    headers.read(b"X-Empty:").unwrap();

    assert_eq!(headers.get("x-empty"), Some(""));
}

#[test]
fn test_read_with_only_spaces_after_colon() {
    let mut headers = Headers::new();
    headers.read(b"X-Spaces:   ").unwrap();

    assert_eq!(headers.get("x-spaces"), Some(""));
}

#[test]
fn test_read_invalid_utf8() {
    let mut headers = Headers::new();
    // Invalid UTF-8 sequence
    let invalid_bytes = b"Content-Type: \xFF\xFE";

    let result = headers.read(invalid_bytes);
    assert!(result.is_err());
}

#[test]
fn test_read_multiple_headers_sequentially() {
    let mut headers = Headers::new();
    headers.read(b"Content-Type: application/json").unwrap();
    headers.read(b"Accept: text/html").unwrap();
    headers.read(b"X-Custom-Header: custom-value").unwrap();

    assert_eq!(headers.get("content-type"), Some("application/json"));
    assert_eq!(headers.get("accept"), Some("text/html"));
    assert_eq!(headers.get("x-custom-header"), Some("custom-value"));
}

#[test]
fn test_read_same_header_multiple_times() {
    let mut headers = Headers::new();
    headers.read(b"Accept: text/html").unwrap();
    headers.read(b"Accept: application/json").unwrap();

    // get() should return the first value
    assert_eq!(headers.get("accept"), Some("text/html"));
}

// Integration tests: read then write
#[test]
fn test_read_write_roundtrip_single_header() {
    let mut headers = Headers::new();
    headers.read(b"Content-Type: application/json").unwrap();

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "content-type: application/json\r\n");
}

#[test]
fn test_read_write_roundtrip_multiple_headers() {
    let mut headers = Headers::new();
    headers.read(b"Content-Type: application/json").unwrap();
    headers.read(b"Accept: text/html").unwrap();

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();

    // Check both headers are present in output
    assert!(output.contains("content-type: application/json\r\n"));
    assert!(output.contains("accept: text/html\r\n"));
}

#[test]
fn test_add_write_single_header() {
    let mut headers = Headers::new();
    headers.add("User-Agent", "Mozilla/5.0");

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "user-agent: Mozilla/5.0\r\n");
}

#[test]
fn test_mixed_add_read_write() {
    let mut headers = Headers::new();
    headers.add("Content-Type", "application/json");
    headers.read(b"Accept: text/html").unwrap();
    headers.add("X-Custom", "value1");

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();

    // Check all headers are present
    assert!(output.contains("content-type: application/json\r\n"));
    assert!(output.contains("accept: text/html\r\n"));
    assert!(output.contains("x-custom: value1\r\n"));
}
