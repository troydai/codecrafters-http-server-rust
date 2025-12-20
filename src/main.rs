mod consts;
mod request;
mod response;
mod router;

#[allow(unused_imports)]
use anyhow::Result;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // You can use print statements as follows for debugging,
    // they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(e) = process_stream(&mut stream) {
                    println!("failed connection: {e}");
                }
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
        router::internal_err_response(&req).write(stream)?;
    }

    Ok(())
}
