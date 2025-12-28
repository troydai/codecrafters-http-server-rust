mod body;
mod connection;
mod consts;
mod file;
mod header;
mod http;
mod request;
mod response;
mod router;
mod server;

#[allow(unused_imports)]
use anyhow::Result;
use clap::Parser;
use std::net::TcpListener;

fn main() -> Result<()> {
    let arg = Args::parse();
    let file_server = file::create(arg.directory)?;
    let router = router::Router::new(file_server);
    let server = server::HttpServer::new(router);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server started at 127.0.0.1:4221");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                server.handle(stream);
            }
            Err(e) => {
                println!("error: {e}");
            }
        }

        println!("finished processing the connection");
    }

    Ok(())
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    directory: Option<String>,
}
