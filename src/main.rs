mod request;

#[allow(unused_imports)]
use anyhow::Result;
use std::io::Read;
use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

use request::Request;

const CRLF: &[u8] = b"\r\n";

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
    let req = read_request(stream)?;
    // println!("Received {:?}", req);

    if req.path == "/" {
        write_response(
            stream,
            &Response::new(&req.protocol, String::from("200"), String::from("OK")),
        )?;
    } else {
        write_response(
            stream,
            &Response::new(
                &req.protocol,
                String::from("404"),
                String::from("Not Found"),
            ),
        )?;
    }

    Ok(())
}

#[derive(Debug)]
struct Response {
    protocol: String,
    status_code: String,
    status_phrase: String,
}

impl Response {
    pub fn new(protocol: &str, status_code: String, status_phrase: String) -> Self {
        Self {
            protocol: String::from(protocol),
            status_code,
            status_phrase,
        }
    }
}

fn read_request(stream: &mut TcpStream) -> Result<request::Request> {
    let mut line: Vec<u8> = Vec::new();
    loop {
        let mut buf = [0; 1024];
        let size = stream.read(&mut buf)?;

        if let Some(pos) = buf[..size].windows(2).position(|w| w == CRLF) {
            line.extend(&buf[..pos]);
            break;
        } else {
            line.extend(&buf);
        }
    }

    Request::from_header(std::str::from_utf8(&line[..])?)
}

fn write_response(stream: &mut TcpStream, resp: &Response) -> Result<()> {
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
