#![allow(unused_imports)]
use std::{
    fs,
    io::{prelude::*, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    string,
};

struct Headers {
    request_api_key: i16,
    request_api_version: i16,
    correlation_id: i32,
    client_id: Option<String>,
    tagged_fields: Option<String>,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line = lines.next().unwrap().unwrap();
    print!("{:?}", request_line);
    let response: i64 = 2_i64.pow(32) + 7_i64;
    stream.write_all(&response.to_be_bytes()).unwrap();
}
