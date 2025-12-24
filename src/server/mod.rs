/*
 * This module defines a HttpServer that handles connection.
 */

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
        let req = request::from_reader(&mut stream)?;
        let resp = router.handle(&req)?;
        resp.write(&mut stream)?;
        Ok(())
    }
}
