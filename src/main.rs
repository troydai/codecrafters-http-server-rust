#[allow(unused_imports)]
use anyhow::Result;
use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(e) = write_response(&mut stream) {
                    println!("failed to write response: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }

        println!("finished processing the connection");
    }
}

fn write_response(stream: &mut TcpStream) -> Result<()> {
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
    stream.flush()?;
    Ok(())
}