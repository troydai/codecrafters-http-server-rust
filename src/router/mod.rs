use crate::consts;
use crate::file;
use crate::response::Response;
use crate::{request::Request, response};

use anyhow::{Ok, Result};

pub struct Router {
    file_server: Box<dyn file::FileRetriever + Send + Sync>,
}

impl Router {
    pub fn new(file_server: Box<dyn file::FileRetriever + Send + Sync>) -> Self {
        Self { file_server }
    }

    pub fn handle(&self, req: &Request) -> Result<Response> {
        if req.path_match_exact("/") {
            return Ok(response::ok());
        }

        if req.path_match_prefix("/echo/") {
            let message = &req.path()?[6..];
            let mut resp = response::ok();
            resp.set_str_body(message);
            return Ok(resp);
        }

        if req.path_match_exact("/user-agent") {
            if let Some(value) = req.headers().get(consts::HEADER_USER_AGENT) {
                let mut resp = response::ok();
                resp.set_str_body(value);

                return Ok(resp);
            }

            return Ok(response::bad_request("missing user-agent header"));
        }

        if req.path_match_prefix("/files") {
            let path = &req.path()?[7..];
            if path.contains("..") || path.starts_with('/') {
                return Ok(response::Response::new(
                    crate::http::status::HttpStatus::Forbidden,
                ));
            }

            let content = self.file_server.retrieve(path)?;
            let mut resp = response::ok();
            resp.set_bytes_body("application/octet-stream", &content);
            return Ok(resp);
        }

        Ok(response::not_found())
    }
}
