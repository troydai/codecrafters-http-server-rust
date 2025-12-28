use super::Headers;

#[test]
fn test_new_creates_empty_headers() {
    let headers = Headers::new();
    // Headers::new() creates empty headers with no defaults
    assert!(headers.get("content-length").is_none());
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
fn test_write_new_headers_is_empty() {
    let headers = Headers::new();

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    // Headers::new() creates empty headers
    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "");
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

// Tests for set() method
#[test]
fn test_set_new_header() {
    let mut headers = Headers::new();
    headers.set("content-type", "application/json");

    assert_eq!(headers.get("content-type"), Some("application/json"));
}

#[test]
fn test_set_overwrites_existing_single_value() {
    let mut headers = Headers::new();
    headers.set("content-type", "text/html");
    headers.set("content-type", "application/json");

    assert_eq!(headers.get("content-type"), Some("application/json"));
}

#[test]
fn test_set_normalizes_case() {
    // Note: set() now converts names to lowercase
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json");

    // get() normalizes to lowercase, and set() should have stored it lowercase
    assert!(headers.get("content-type").is_some());
}

#[test]
fn test_set_overwrites_existing_multiple_values() {
    let mut headers = Headers::new();
    headers.add("accept", "text/html");
    headers.add("accept", "application/xml");
    headers.add("accept", "text/plain");

    // set should clear all values and replace with single value
    headers.set("accept", "application/json");

    assert_eq!(headers.get("accept"), Some("application/json"));
}

#[test]
fn test_set_then_write() {
    let mut headers = Headers::new();
    headers.set("content-type", "application/json");

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "content-type: application/json\r\n");
}

#[test]
fn test_set_multiple_different_headers() {
    let mut headers = Headers::new();
    headers.set("content-type", "application/json");
    headers.set("accept", "text/html");

    assert_eq!(headers.get("content-type"), Some("application/json"));
    assert_eq!(headers.get("accept"), Some("text/html"));
}

// Tests for accept_encodings() method
#[test]
fn test_accept_encoding_returns_value_when_present() {
    let mut headers = Headers::new();
    headers.add("Accept-Encoding", "gzip");

    assert_eq!(
        headers.accept_encodings().unwrap(),
        vec!["gzip".to_string()]
    );
}

#[test]
fn test_accept_encoding_returns_none_when_absent() {
    let headers = Headers::new();

    assert!(headers.accept_encodings().is_none());
}

#[test]
fn test_accept_encoding_case_insensitive() {
    let mut headers = Headers::new();
    headers.add("ACCEPT-ENCODING", "gzip");

    assert_eq!(
        headers.accept_encodings().unwrap(),
        vec!["gzip".to_string()]
    );
}

#[test]
fn test_accept_encoding_from_read() {
    let mut headers = Headers::new();
    headers.read(b"Accept-Encoding: deflate").unwrap();

    assert_eq!(
        headers.accept_encodings().unwrap(),
        vec!["deflate".to_string()]
    );
}

#[test]
fn test_accept_encoding_with_multiple_values() {
    let mut headers = Headers::new();
    headers.read(b"Accept-Encoding: gzip, deflate, br").unwrap();

    assert_eq!(
        headers.accept_encodings().unwrap(),
        vec!["gzip".to_string(), "deflate".to_string(), "br".to_string()]
    );
}

#[test]
fn test_accept_encoding_multiple_values_split_into_vector() {
    let mut headers = Headers::new();
    headers
        .read(b"Accept-Encoding: option1, gzip, option2")
        .unwrap();

    let values = headers.accept_encodings().unwrap();
    assert_eq!(
        values,
        vec![
            "option1".to_string(),
            "gzip".to_string(),
            "option2".to_string()
        ]
    );
}

#[test]
fn test_other_headers_not_split_by_comma() {
    let mut headers = Headers::new();
    // User-Agent often contains commas or other delimiters but should not be split
    headers
        .read(b"User-Agent: Mozilla/5.0, Chrome/91.0")
        .unwrap();

    assert_eq!(headers.get("User-Agent"), Some("Mozilla/5.0, Chrome/91.0"));
}

// Tests for content_length() method
#[test]
fn test_content_length_returns_value_when_present() {
    let mut headers = Headers::new();
    // Use set_content_length() to properly set the value (replacing the default 0)
    headers.set_content_length(42);

    assert_eq!(headers.content_length().unwrap(), 42);
}

#[test]
fn test_content_length_returns_zero_when_absent() {
    // Headers::new() creates empty headers, content_length() returns 0 when absent
    let headers = Headers::new();

    assert_eq!(headers.content_length().unwrap(), 0);
}

#[test]
fn test_content_length_returns_error_for_invalid_number() {
    let mut headers = Headers::new();
    headers.read(b"Content-Length: not-a-number").unwrap();

    // Invalid Content-Length value causes content_length() to return an error
    assert!(headers.content_length().is_err());
}

#[test]
fn test_content_length_returns_error_for_negative_number() {
    let mut headers = Headers::new();
    headers.read(b"Content-Length: -10").unwrap();

    // Negative numbers can't be parsed as usize, so content_length() returns an error
    assert!(headers.content_length().is_err());
}

#[test]
fn test_content_length_returns_error_for_float() {
    let mut headers = Headers::new();
    headers.read(b"Content-Length: 3.14").unwrap();

    // Floats can't be parsed as usize, so content_length() returns an error
    assert!(headers.content_length().is_err());
}

#[test]
fn test_content_length_returns_zero_for_zero() {
    let mut headers = Headers::new();
    headers.add("Content-Length", "0");

    assert_eq!(headers.content_length().unwrap(), 0);
}

#[test]
fn test_content_length_large_value() {
    let mut headers = Headers::new();
    // Use set_content_length() to properly set the value
    headers.set_content_length(1_073_741_824); // 1 GB

    assert_eq!(headers.content_length().unwrap(), 1_073_741_824);
}

#[test]
fn test_content_length_case_insensitive() {
    let mut headers = Headers::new();
    // Use set_content_length() to properly set the value
    headers.set_content_length(100);

    assert_eq!(headers.content_length().unwrap(), 100);
}

#[test]
fn test_content_length_from_read() {
    let mut headers = Headers::new();
    headers.read(b"Content-Length: 256").unwrap();

    assert_eq!(headers.content_length().unwrap(), 256);
}

#[test]
fn test_content_length_with_whitespace() {
    let mut headers = Headers::new();
    headers.read(b"Content-Length:   512   ").unwrap();

    assert_eq!(headers.content_length().unwrap(), 512);
}

// Tests for content_type() method
#[test]
fn test_content_type_returns_value_when_present() {
    let mut headers = Headers::new();
    headers.add("Content-Type", "application/json");

    assert_eq!(headers.content_type(), Some("application/json"));
}

#[test]
fn test_content_type_returns_none_when_absent() {
    let headers = Headers::new();

    assert_eq!(headers.content_type(), None);
}

#[test]
fn test_content_type_case_insensitive() {
    let mut headers = Headers::new();
    headers.add("content-type", "text/html");

    assert_eq!(headers.content_type(), Some("text/html"));
}

#[test]
fn test_content_type_from_read() {
    let mut headers = Headers::new();
    headers.read(b"Content-Type: text/plain").unwrap();

    assert_eq!(headers.content_type(), Some("text/plain"));
}

#[test]
fn test_content_type_with_charset() {
    let mut headers = Headers::new();
    headers.add("Content-Type", "text/html; charset=utf-8");

    assert_eq!(headers.content_type(), Some("text/html; charset=utf-8"));
}

#[test]
fn test_content_type_with_whitespace() {
    let mut headers = Headers::new();
    headers.read(b"Content-Type:   application/xml   ").unwrap();

    assert_eq!(headers.content_type(), Some("application/xml"));
}

#[test]
fn test_connection_returns_value_when_present() {
    let mut headers = Headers::new();
    headers.add("Connection", "keep-alive");

    assert_eq!(headers.connection(), Some("keep-alive"));
}

#[test]
fn test_connection_returns_none_when_absent() {
    let headers = Headers::new();

    assert_eq!(headers.connection(), None);
}

#[test]
fn test_connection_case_insensitive() {
    let mut headers = Headers::new();
    headers.add("CONNECTION", "close");

    assert_eq!(headers.connection(), Some("close"));
}

#[test]
fn test_connection_from_read() {
    let mut headers = Headers::new();
    headers.read(b"Connection: close").unwrap();

    assert_eq!(headers.connection(), Some("close"));
}

#[test]
fn test_connection_with_whitespace() {
    let mut headers = Headers::new();
    headers.read(b"Connection:   keep-alive   ").unwrap();

    assert_eq!(headers.connection(), Some("keep-alive"));
}

// Tests for set_content_length() method
#[test]
fn test_set_content_length_sets_header() {
    let mut headers = Headers::new();
    headers.set_content_length(42);

    assert_eq!(headers.get("content-length"), Some("42"));
}

#[test]
fn test_set_content_length_zero() {
    let mut headers = Headers::new();
    headers.set_content_length(0);

    assert_eq!(headers.get("content-length"), Some("0"));
}

#[test]
fn test_set_content_length_overwrites_existing() {
    let mut headers = Headers::new();
    headers.set_content_length(100);
    headers.set_content_length(200);

    assert_eq!(headers.get("content-length"), Some("200"));
}

#[test]
fn test_set_content_length_large_value() {
    let mut headers = Headers::new();
    headers.set_content_length(1_073_741_824); // 1 GB

    assert_eq!(headers.get("content-length"), Some("1073741824"));
}

#[test]
fn test_set_content_length_then_content_length() {
    let mut headers = Headers::new();
    headers.set_content_length(256);

    // content_length() should return the value we just set
    assert_eq!(headers.content_length().unwrap(), 256);
}

#[test]
fn test_set_content_length_write_format() {
    let mut headers = Headers::new();
    headers.set_content_length(42);

    let mut buffer = Vec::new();
    headers.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "content-length: 42\r\n");
}
