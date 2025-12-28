use super::*;
use crate::http::status::HttpStatus;
use crate::response::Response;
use flate2::read::GzDecoder;
use std::io::Read;

#[test]
fn test_gzip_compression() {
    let mut resp = Response::new(HttpStatus::Ok);
    let original_body = "Hello, world! This is a test string to be compressed.";
    resp.set_str_body(original_body);

    // Call the compression function (which we will implement)
    compress_response_body(&mut resp).unwrap();

    // Verify headers
    assert_eq!(resp.headers().get("Content-Encoding"), Some("gzip"));

    // Extract body and decompress it
    if let crate::body::HttpBody::Content(compressed_bytes) = resp.body() {
        let mut decoder = GzDecoder::new(&compressed_bytes[..]);
        let mut decompressed_body = String::new();
        decoder.read_to_string(&mut decompressed_body).unwrap();
        assert_eq!(decompressed_body, original_body);

        // Verify Content-Length is updated to compressed size
        assert_eq!(
            resp.headers().content_length().unwrap(),
            compressed_bytes.len()
        );
    } else {
        panic!("Response body should be Content");
    }
}
