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
    resp.set_body("Hello, World!");

    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.ends_with("Hello, World!"));
}

#[test]
fn test_set_body_sets_content_type() {
    let mut resp = Response::new(HttpStatus::Ok);
    resp.set_body("test");

    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("Content-Type: text/plain\r\n"));
}

#[test]
fn test_set_body_sets_content_length() {
    let mut resp = Response::new(HttpStatus::Ok);
    resp.set_body("Hello");

    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("Content-Length: 5\r\n"));
}

// Tests for Response::write()
#[test]
fn test_write_format_with_body() {
    let mut resp = Response::new(HttpStatus::Ok);
    resp.set_body("test body");

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
    assert_eq!(output, "HTTP/1.1 404 Not Found\r\n\r\n");
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
fn test_internal_err_response_returns_500() {
    let resp = internal_err_response();
    let mut buffer = Vec::new();
    resp.write(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert!(output.starts_with("HTTP/1.1 500 Internal Server Error\r\n"));
}

// Tests for HttpStatus::write_request_line()
#[test]
fn test_status_write_request_line_ok() {
    let status = HttpStatus::Ok;
    let mut buffer = Vec::new();
    status.write_request_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 200 OK\r\n");
}

#[test]
fn test_status_write_request_line_bad_request() {
    let status = HttpStatus::BadRequest;
    let mut buffer = Vec::new();
    status.write_request_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 400 Bad Request\r\n");
}

#[test]
fn test_status_write_request_line_unauthorized() {
    let status = HttpStatus::Unauthorized;
    let mut buffer = Vec::new();
    status.write_request_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 401 Unauthorized\r\n");
}

#[test]
fn test_status_write_request_line_forbidden() {
    let status = HttpStatus::Forbidden;
    let mut buffer = Vec::new();
    status.write_request_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 403 Forbidden\r\n");
}

#[test]
fn test_status_write_request_line_not_found() {
    let status = HttpStatus::NotFound;
    let mut buffer = Vec::new();
    status.write_request_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 404 Not Found\r\n");
}

#[test]
fn test_status_write_request_line_internal_server_error() {
    let status = HttpStatus::InternalServerError;
    let mut buffer = Vec::new();
    status.write_request_line(&mut buffer).unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output, "HTTP/1.1 500 Internal Server Error\r\n");
}
