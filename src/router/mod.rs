use crate::request::Request;
use crate::response::Response;

use anyhow::{Ok, Result};

pub fn handle(req: &Request) -> Result<Response> {
    if req.path() == "/" {
        return Ok(Response::new(
            &req.protocol,
            String::from("200"),
            String::from("OK"),
        ));
    }

    if req.path().starts_with("/echo/") {
        let message = &req.path()[6..];
        return Ok(Response::with_body(&req.protocol, message));
    }

    Ok(Response::new(
        &req.protocol,
        String::from("404"),
        String::from("Not Found"),
    ))
}

pub fn internal_err_response(req: &Request) -> Response {
    Response::new(
        &req.protocol,
        String::from("500"),
        String::from("Internal Server Error"),
    )
}
