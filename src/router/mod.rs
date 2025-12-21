use crate::response::Response;
use crate::{request::Request, response};

use anyhow::{Ok, Result};

pub fn handle(req: &Request) -> Result<Response> {
    if req.path() == "/" {
        return Ok(response::ok());
    }

    if req.path().starts_with("/echo/") {
        let message = &req.path()[6..];
        let resp = Response::with_body(message);
        return Ok(resp);
    }

    if req.path() == "/user-agent" {
        if let Some(values) = req.headers().value("User-Agent") {
            let resp = response::Response::with_body(values.first_value());
            return Ok(resp);
        }

        return Ok(response::bad_request("missing user-agent header"));
    }

    Ok(response::not_found())
}
