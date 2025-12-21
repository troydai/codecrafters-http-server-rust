mod consts;
mod header;
mod http;
mod request;
mod response;
mod router;

#[allow(unused_imports)]
use anyhow::Result;
use std::net::TcpListener;
use std::net::TcpStream;
use threadpool::ThreadPool;

fn main() {
    // You can use print statements as follows for debugging,
    // they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let thread_pool = ThreadPool::new(16);

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread_pool.execute(move || {
                    if let Err(e) = process_stream(&mut stream) {
                        println!("failed connection: {e}");
                    }
                });
            }
            Err(e) => {
                println!("error: {e}");
            }
        }

        println!("finished processing the connection");
    }
}

fn process_stream(stream: &mut TcpStream) -> Result<()> {
    let req = request::from_reader(stream)?;

    if let Ok(resp) = router::handle(&req) {
        resp.write(stream)?;
    } else {
        response::internal_err_response().write(stream)?;
    }

    Ok(())
}
