use super::Headers;

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
