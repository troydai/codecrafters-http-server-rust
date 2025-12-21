use crate::consts;
use crate::response::Response;
use crate::{request::Request, response};

use anyhow::{Ok, Result};

pub fn handle(req: &Request) -> Result<Response> {
    if req.path_match_exact("/") {
        return Ok(response::ok());
    }

    if req.path_match_prefix("/echo/") {
        let message = &req.path()?[6..];
        let mut resp = response::ok();
        resp.set_body(message);
        return Ok(resp);
    }

    if req.path_match_exact("/user-agent") {
        if let Some(value) = req.headers().get(consts::HEADER_USER_AGENT) {
            let mut resp = response::ok();
            resp.set_body(value);

            return Ok(resp);
        }

        return Ok(response::bad_request("missing user-agent header"));
    }

    Ok(response::not_found())
}
