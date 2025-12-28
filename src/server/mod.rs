/*
 * This module defines a HttpServer that handles connection.
 */

use crate::connection::LineStream;
use crate::request;
use crate::router::Router;
use anyhow::Result;
use std::net::TcpStream;
use std::sync::Arc;
use threadpool::ThreadPool;

pub struct HttpServer {
    router: Arc<Router>,
    pool: ThreadPool,
}

impl HttpServer {
    pub fn new(router: Router) -> Self {
        Self {
            router: Arc::new(router),
            pool: ThreadPool::new(16),
        }
    }

    /// Handle a TCP connection by dispatching it to the thread pool.
    pub fn handle(&self, stream: TcpStream) {
        let router = Arc::clone(&self.router);
        self.pool.execute(move || {
            if let Err(e) = Self::handle_connection(&router, stream) {
                eprintln!("error handling connection: {e}");
            }
        });
    }

    fn handle_connection(router: &Arc<Router>, mut stream: TcpStream) -> Result<()> {
        let remote_addr = &stream.peer_addr()?;
        println!("Accepted connection from {remote_addr:?}");

        let mut line_stream = LineStream::new(&mut stream);

        loop {
            println!("Start handling request from {remote_addr:?}");
            // Try to read the next request; break if client closed connection or error occurred
            let Some(req) = request::from_line_stream(&mut line_stream).ok() else {
                break;
            };

            // Check if client requested connection close
            let should_close = req
                .headers()
                .connection()
                .is_some_and(|v| v.eq_ignore_ascii_case("close"));

            // Handle the request and write response
            let resp = router.handle(&req)?;
            resp.write(&mut line_stream)?;

            // Close connection if requested
            if should_close {
                break;
            }
        }

        println!("Finish handling connection from {remote_addr:?}");
        Ok(())
    }
}
