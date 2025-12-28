use super::*;

// Tests for Response::new()
#[test]
fn test_new_creates_response_with_status() {
    let resp = Response::new(HttpStatus::Ok);
    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.starts_with("HTTP/1.1 200 OK\r\n"));
}

#[test]
fn test_new_response_has_no_body() {
    let resp = Response::new(HttpStatus::Ok);
    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    // Should end with empty line (header/body separator) and nothing after
    assert!(output.ends_with("\r\n\r\n"));
}

// Tests for Response::set_body()
#[test]
fn test_set_body_adds_content() {
    let mut resp = Response::new(HttpStatus::Ok);
    resp.set_str_body("Hello, World!");

    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.ends_with("Hello, World!"));
}

#[test]
fn test_set_body_sets_content_type() {
    let mut resp = Response::new(HttpStatus::Ok);
    resp.set_str_body("test");

    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("Content-Type: text/plain\r\n"));
}

#[test]
fn test_set_body_sets_content_length() {
    let mut resp = Response::new(HttpStatus::Ok);
    resp.set_str_body("Hello");

    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    // Content-Length is written via Headers which uses lowercase keys
    assert!(output.contains("content-length: 5\r\n"));
}

// Tests for Response::write()
#[test]
fn test_write_format_with_body() {
    let mut resp = Response::new(HttpStatus::Ok);
    resp.set_str_body("test body");

    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();

    // Check structure: status line, headers, empty line, body
    assert!(output.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(output.contains("\r\n\r\n")); // empty line separator
    assert!(output.ends_with("test body"));
}

#[test]
fn test_write_format_without_body() {
    let resp = Response::new(HttpStatus::NotFound);

    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    // Responses without body must include Content-Length: 0 for HTTP/1.1 persistent connections
    // The header is written via Headers struct which normalizes names to lowercase
    assert_eq!(
        output,
        "HTTP/1.1 404 Not Found\r\ncontent-length: 0\r\n\r\n"
    );
}

// Tests for factory functions
#[test]
fn test_ok_returns_200() {
    let resp = ok();
    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.starts_with("HTTP/1.1 200 OK\r\n"));
}

#[test]
fn test_not_found_returns_404() {
    let resp = not_found();
    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.starts_with("HTTP/1.1 404 Not Found\r\n"));
}

#[test]
fn test_bad_request_returns_400_with_body() {
    let resp = bad_request("Invalid input");
    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.starts_with("HTTP/1.1 400 Bad Request\r\n"));
    assert!(output.ends_with("Invalid input"));
}

#[test]
fn test_internal_server_error_returns_500_without_message() {
    let resp = internal_server_error(None);
    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.starts_with("HTTP/1.1 500 Internal Server Error\r\n"));
    assert!(output.ends_with("\r\n\r\n")); // no body
}

#[test]
fn test_internal_server_error_returns_500_with_message() {
    let resp = internal_server_error(Some("Something went wrong"));
    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.starts_with("HTTP/1.1 500 Internal Server Error\r\n"));
    assert!(output.ends_with("Something went wrong"));
}

// Tests for HttpStatus::write_status_line()
#[test]
fn test_status_write_status_line_ok() {
    let status = HttpStatus::Ok;
    let mut buffer = Vec::new();
    status.write_status_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 200 OK\r\n");
}

#[test]
fn test_status_write_status_line_bad_request() {
    let status = HttpStatus::BadRequest;
    let mut buffer = Vec::new();
    status.write_status_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 400 Bad Request\r\n");
}

#[test]
fn test_status_write_status_line_unauthorized() {
    let status = HttpStatus::Unauthorized;
    let mut buffer = Vec::new();
    status.write_status_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 401 Unauthorized\r\n");
}

#[test]
fn test_status_write_status_line_forbidden() {
    let status = HttpStatus::Forbidden;
    let mut buffer = Vec::new();
    status.write_status_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 403 Forbidden\r\n");
}

#[test]
fn test_status_write_status_line_not_found() {
    let status = HttpStatus::NotFound;
    let mut buffer = Vec::new();
    status.write_status_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 404 Not Found\r\n");
}

#[test]
fn test_status_write_status_line_internal_server_error() {
    let status = HttpStatus::InternalServerError;
    let mut buffer = Vec::new();
    status.write_status_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 500 Internal Server Error\r\n");
}

// Tests for Response headers consistency
#[test]
fn test_response_without_body_uses_headers_for_content_length() {
    let resp = Response::new(HttpStatus::Ok);
    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    // Content-Length should be present via Headers struct (lowercase key format)
    assert!(output.contains("content-length: 0\r\n"));
}

#[test]
fn test_response_with_body_uses_headers_for_content_length() {
    let mut resp = Response::new(HttpStatus::Ok);
    resp.set_str_body("Hello");

    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    // Content-Length should be written consistently via Headers struct
    // Note: set_str_body uses headers.set() which preserves case,
    // so we need to check the actual behavior
    assert!(
        output.contains("Content-Length: 5\r\n") || output.contains("content-length: 5\r\n"),
        "Expected Content-Length header, got: {}",
        output
    );
}

// Tests for Response owning Headers and auto-updating Content-Length
#[test]
fn test_new_response_has_content_length_zero_in_headers() {
    // Response::new() should initialize headers with Content-Length: 0
    // The headers should be stored in the struct, not created during write()
    let resp = Response::new(HttpStatus::Ok);

    // Verify the Content-Length header is actually stored (not just returning default 0)
    // by checking that get() returns Some, not None
    assert!(
        resp.headers().get("Content-Length").is_some(),
        "Content-Length header should be explicitly stored in new Response"
    );
    assert_eq!(
        resp.headers().content_length().unwrap(),
        0,
        "New response should have Content-Length: 0 in headers"
    );
}

#[test]
fn test_set_str_body_updates_content_length_in_headers() {
    let mut resp = Response::new(HttpStatus::Ok);
    resp.set_str_body("Hello");

    // Verify via headers() that Content-Length was updated
    assert_eq!(
        resp.headers().content_length().unwrap(),
        5,
        "Content-Length should be updated to body length"
    );
}

#[test]
fn test_set_bytes_body_updates_content_length_in_headers() {
    let mut resp = Response::new(HttpStatus::Ok);
    let body = b"binary data here";
    resp.set_bytes_body("application/octet-stream", body);

    // Verify via headers() that Content-Length was updated
    assert_eq!(
        resp.headers().content_length().unwrap(),
        body.len(),
        "Content-Length should be updated to body length"
    );
}

#[test]
fn test_response_headers_are_owned_not_created_during_write() {
    // This test verifies that headers are stored in the struct,
    // not created on-the-fly during write()
    let resp = Response::new(HttpStatus::Ok);
    let headers = resp.headers();

    // Should be able to access headers before calling write()
    assert_eq!(headers.content_length().unwrap(), 0);
}

#[test]
fn test_set_body_multiple_times_updates_content_length() {
    let mut resp = Response::new(HttpStatus::Ok);

    resp.set_str_body("short");
    assert_eq!(resp.headers().content_length().unwrap(), 5);

    resp.set_str_body("a much longer body");
    assert_eq!(resp.headers().content_length().unwrap(), 18);
}
