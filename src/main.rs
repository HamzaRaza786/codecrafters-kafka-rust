#![allow(unused_imports)]
use bytes::{Buf, BufMut};
use std::{
    any, fs,
    io::{prelude::*, BufReader, Cursor, Read, Seek, Write},
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
    // let mut buf_reader = BufReader::new(&mut stream);

    let mut len = [0; 4];
    stream.read_exact(&mut len).unwrap();
    let len = i32::from_be_bytes(len) as usize;
    let mut request = vec![0; len];
    stream.read_exact(&mut request).unwrap();
    let mut request = request.as_slice();
    let _request_api_key = request.get_i16();
    let _request_api_version = request.get_i16();
    let correlation_id = request.get_i32();
    let mut response = Vec::with_capacity(8);
    response.put_i32(0);
    response.put_i32(correlation_id);
    stream.write_all(&response).unwrap();
}
