use super::method::HttpMethod;
use super::status::HttpStatus;

#[test]
fn test_parse_get() {
    let method: HttpMethod = "GET".parse().unwrap();
    assert_eq!(method, HttpMethod::Get);
}

#[test]
fn test_parse_post() {
    let method: HttpMethod = "POST".parse().unwrap();
    assert_eq!(method, HttpMethod::Post);
}

#[test]
fn test_parse_put() {
    let method: HttpMethod = "PUT".parse().unwrap();
    assert_eq!(method, HttpMethod::Put);
}

#[test]
fn test_parse_delete() {
    let method: HttpMethod = "DELETE".parse().unwrap();
    assert_eq!(method, HttpMethod::Delete);
}

#[test]
fn test_parse_patch() {
    let method: HttpMethod = "PATCH".parse().unwrap();
    assert_eq!(method, HttpMethod::Patch);
}

#[test]
fn test_parse_head() {
    let method: HttpMethod = "HEAD".parse().unwrap();
    assert_eq!(method, HttpMethod::Head);
}

#[test]
fn test_parse_options() {
    let method: HttpMethod = "OPTIONS".parse().unwrap();
    assert_eq!(method, HttpMethod::Options);
}

#[test]
fn test_parse_connect() {
    let method: HttpMethod = "CONNECT".parse().unwrap();
    assert_eq!(method, HttpMethod::Connect);
}

#[test]
fn test_parse_trace() {
    let method: HttpMethod = "TRACE".parse().unwrap();
    assert_eq!(method, HttpMethod::Trace);
}

#[test]
fn test_parse_invalid_method() {
    let result = "INVALID".parse::<HttpMethod>();
    assert!(result.is_err());
}

#[test]
fn test_parse_lowercase_get_fails() {
    let result = "get".parse::<HttpMethod>();
    assert!(result.is_err());
}

#[test]
fn test_parse_lowercase_post_fails() {
    let result = "post".parse::<HttpMethod>();
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_string_fails() {
    let result = "".parse::<HttpMethod>();
    assert!(result.is_err());
}

#[test]
fn test_method_equality() {
    let get1: HttpMethod = "GET".parse().unwrap();
    let get2: HttpMethod = "GET".parse().unwrap();
    assert_eq!(get1, get2);
}

#[test]
fn test_method_inequality() {
    let get: HttpMethod = "GET".parse().unwrap();
    let post: HttpMethod = "POST".parse().unwrap();
    assert_ne!(get, post);
}

#[test]
fn test_method_clone() {
    let original = HttpMethod::Get;
    let cloned = original;
    assert_eq!(original, cloned);
}

#[test]
fn test_method_debug() {
    let method = HttpMethod::Get;
    let debug_str = format!("{method:?}");
    assert_eq!(debug_str, "Get");
}

// HttpStatus tests

#[test]
fn test_status_ok_write_status_line() {
    let mut buffer = Vec::new();
    HttpStatus::Ok.write_status_line(&mut buffer).unwrap();
    assert_eq!(buffer, b"HTTP/1.1 200 OK\r\n");
}

#[test]
fn test_status_bad_request_write_status_line() {
    let mut buffer = Vec::new();
    HttpStatus::BadRequest
        .write_status_line(&mut buffer)
        .unwrap();
    assert_eq!(buffer, b"HTTP/1.1 400 Bad Request\r\n");
}

#[test]
fn test_status_unauthorized_write_status_line() {
    let mut buffer = Vec::new();
    HttpStatus::Unauthorized
        .write_status_line(&mut buffer)
        .unwrap();
    assert_eq!(buffer, b"HTTP/1.1 401 Unauthorized\r\n");
}

#[test]
fn test_status_forbidden_write_status_line() {
    let mut buffer = Vec::new();
    HttpStatus::Forbidden
        .write_status_line(&mut buffer)
        .unwrap();
    assert_eq!(buffer, b"HTTP/1.1 403 Forbidden\r\n");
}

#[test]
fn test_status_not_found_write_status_line() {
    let mut buffer = Vec::new();
    HttpStatus::NotFound.write_status_line(&mut buffer).unwrap();
    assert_eq!(buffer, b"HTTP/1.1 404 Not Found\r\n");
}

#[test]
fn test_status_internal_server_error_write_status_line() {
    let mut buffer = Vec::new();
    HttpStatus::InternalServerError
        .write_status_line(&mut buffer)
        .unwrap();
    assert_eq!(buffer, b"HTTP/1.1 500 Internal Server Error\r\n");
}

#[test]
fn test_status_debug() {
    assert_eq!(format!("{:?}", HttpStatus::Ok), "Ok");
    assert_eq!(format!("{:?}", HttpStatus::NotFound), "NotFound");
}

#[test]
fn test_status_no_content_write_status_line() {
    let mut buffer = Vec::new();
    HttpStatus::NoContent
        .write_status_line(&mut buffer)
        .unwrap();
    assert_eq!(buffer, b"HTTP/1.1 204 No Content\r\n");
}

#[test]
fn test_status_enum_values() {
    assert_eq!(HttpStatus::Ok as u16, 200);
    assert_eq!(HttpStatus::NoContent as u16, 204);
    assert_eq!(HttpStatus::BadRequest as u16, 400);
    assert_eq!(HttpStatus::Unauthorized as u16, 401);
    assert_eq!(HttpStatus::Forbidden as u16, 403);
    assert_eq!(HttpStatus::NotFound as u16, 404);
    assert_eq!(HttpStatus::InternalServerError as u16, 500);
}
