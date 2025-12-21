use super::method::HttpMethod;
use super::status::HttpStatus;

#[test]
fn test_from_str_get() {
    let method = HttpMethod::from_str("GET").unwrap();
    assert_eq!(method, HttpMethod::Get);
}

#[test]
fn test_from_str_post() {
    let method = HttpMethod::from_str("POST").unwrap();
    assert_eq!(method, HttpMethod::Post);
}

#[test]
fn test_from_str_put() {
    let method = HttpMethod::from_str("PUT").unwrap();
    assert_eq!(method, HttpMethod::Put);
}

#[test]
fn test_from_str_delete() {
    let method = HttpMethod::from_str("DELETE").unwrap();
    assert_eq!(method, HttpMethod::Delete);
}

#[test]
fn test_from_str_patch() {
    let method = HttpMethod::from_str("PATCH").unwrap();
    assert_eq!(method, HttpMethod::Patch);
}

#[test]
fn test_from_str_head() {
    let method = HttpMethod::from_str("HEAD").unwrap();
    assert_eq!(method, HttpMethod::Head);
}

#[test]
fn test_from_str_options() {
    let method = HttpMethod::from_str("OPTIONS").unwrap();
    assert_eq!(method, HttpMethod::Options);
}

#[test]
fn test_from_str_connect() {
    let method = HttpMethod::from_str("CONNECT").unwrap();
    assert_eq!(method, HttpMethod::Connect);
}

#[test]
fn test_from_str_trace() {
    let method = HttpMethod::from_str("TRACE").unwrap();
    assert_eq!(method, HttpMethod::Trace);
}

#[test]
fn test_from_str_invalid_method() {
    let result = HttpMethod::from_str("INVALID");
    assert!(result.is_err());
}

#[test]
fn test_from_str_lowercase_get_fails() {
    let result = HttpMethod::from_str("get");
    assert!(result.is_err());
}

#[test]
fn test_from_str_lowercase_post_fails() {
    let result = HttpMethod::from_str("post");
    assert!(result.is_err());
}

#[test]
fn test_from_str_empty_string_fails() {
    let result = HttpMethod::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_method_equality() {
    let get1 = HttpMethod::from_str("GET").unwrap();
    let get2 = HttpMethod::from_str("GET").unwrap();
    assert_eq!(get1, get2);
}

#[test]
fn test_method_inequality() {
    let get = HttpMethod::from_str("GET").unwrap();
    let post = HttpMethod::from_str("POST").unwrap();
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
    let debug_str = format!("{:?}", method);
    assert_eq!(debug_str, "Get");
}

// HttpStatus tests

#[test]
fn test_status_ok_write_request_line() {
    let mut buffer = Vec::new();
    HttpStatus::Ok.write_request_line(&mut buffer).unwrap();
    assert_eq!(buffer, b"HTTP/1.1 200 OK\r\n");
}

#[test]
fn test_status_bad_request_write_request_line() {
    let mut buffer = Vec::new();
    HttpStatus::BadRequest.write_request_line(&mut buffer).unwrap();
    assert_eq!(buffer, b"HTTP/1.1 400 Bad Request\r\n");
}

#[test]
fn test_status_unauthorized_write_request_line() {
    let mut buffer = Vec::new();
    HttpStatus::Unauthorized.write_request_line(&mut buffer).unwrap();
    assert_eq!(buffer, b"HTTP/1.1 401 Unauthorized\r\n");
}

#[test]
fn test_status_forbidden_write_request_line() {
    let mut buffer = Vec::new();
    HttpStatus::Forbidden.write_request_line(&mut buffer).unwrap();
    assert_eq!(buffer, b"HTTP/1.1 403 Forbidden\r\n");
}

#[test]
fn test_status_not_found_write_request_line() {
    let mut buffer = Vec::new();
    HttpStatus::NotFound.write_request_line(&mut buffer).unwrap();
    assert_eq!(buffer, b"HTTP/1.1 404 Not Found\r\n");
}

#[test]
fn test_status_internal_server_error_write_request_line() {
    let mut buffer = Vec::new();
    HttpStatus::InternalServerError.write_request_line(&mut buffer).unwrap();
    assert_eq!(buffer, b"HTTP/1.1 500 Internal Server Error\r\n");
}

#[test]
fn test_status_debug() {
    assert_eq!(format!("{:?}", HttpStatus::Ok), "Ok");
    assert_eq!(format!("{:?}", HttpStatus::NotFound), "NotFound");
}

#[test]
fn test_status_enum_values() {
    assert_eq!(HttpStatus::Ok as u16, 200);
    assert_eq!(HttpStatus::BadRequest as u16, 400);
    assert_eq!(HttpStatus::Unauthorized as u16, 401);
    assert_eq!(HttpStatus::Forbidden as u16, 403);
    assert_eq!(HttpStatus::NotFound as u16, 404);
    assert_eq!(HttpStatus::InternalServerError as u16, 500);
}
