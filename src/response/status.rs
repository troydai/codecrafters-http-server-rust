use crate::consts::{CRLF, SPACE};
use anyhow::Result;

const HTTP_1_1: &[u8] = b"HTTP/1.1";

#[derive(Debug)]
#[allow(dead_code)]
pub enum HttpStatus {
    Ok = 200,                  // 200
    BadRequest = 400,          // 400
    Unauthorized = 401,        // 401
    Forbidden = 403,           // 403
    NotFound = 404,            // 404
    InternalServerError = 500, // 500
}

impl HttpStatus {
    pub fn write_request_line(&self, stream: &mut impl std::io::Write) -> Result<()> {
        // A request line of HTTP/1.1 looks like this
        // HTTP/1.1 200 OK
        // HTTP/1.1 404 Not Found

        stream.write_all(HTTP_1_1)?; // default to HTTP 1.1 protocol
        stream.write_all(SPACE)?;
        stream.write_all(self.status_code().as_bytes())?;
        stream.write_all(SPACE)?;
        stream.write_all(self.status_phrase().as_bytes())?;
        stream.write_all(CRLF)?;

        Ok(())
    }

    const fn status_phrase(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
        }
    }

    const fn status_code(&self) -> &'static str {
        match self {
            Self::Ok => "200",
            Self::BadRequest => "400",
            Self::Unauthorized => "401",
            Self::Forbidden => "403",
            Self::NotFound => "404",
            Self::InternalServerError => "500",
        }
    }
}
