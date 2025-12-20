mod consts;
mod request;
mod response;

#[allow(unused_imports)]
use anyhow::Result;
use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

use consts::CRLF;

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
    let req = request::from_stream(stream)?;
    // println!("Received {:?}", req);

    if req.path == "/" {
        write_response(
            stream,
            &response::Response::new(&req.protocol, String::from("200"), String::from("OK")),
        )?;
    } else {
        write_response(
            stream,
            &response::Response::new(
                &req.protocol,
                String::from("404"),
                String::from("Not Found"),
            ),
        )?;
    }

    Ok(())
}

fn write_response(stream: &mut TcpStream, resp: &response::Response) -> Result<()> {
    let head = format!(
        "{} {} {}",
        resp.protocol, resp.status_code, resp.status_phrase
    );

    stream.write_all(head.as_bytes())?;
    stream.write_all(CRLF)?;
    stream.write_all(CRLF)?;
    stream.flush()?;
    Ok(())
}
