#![allow(unused_imports, dead_code)]
use bytes::{Buf, BufMut};
use std::{
    any, fs,
    io::{prelude::*, BufReader, Cursor, Read, Seek, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    string,
};

const RESPONSE_LENGTH: usize = 8;

struct Headers {
    request_api_key: i16,
    request_api_version: i16,
    correlation_id: i32,
    client_id: Option<String>,
    tagged_fields: Option<String>,
}

fn message_length(mut stream: &TcpStream) -> usize {
    let mut buffer = [0; 4];
    stream.read_exact(&mut buffer).unwrap();
    i32::from_be_bytes(buffer) as usize
}

fn request_headers(mut stream: &TcpStream) -> Headers {
    let mut request = vec![0; message_length(&stream)];
    stream.read_exact(&mut request).unwrap();
    let mut request = request.as_slice();
    let request_api_key = request.get_i16();
    let request_api_version = request.get_i16();
    let correlation_id = request.get_i32();
    Headers {
        request_api_key,
        request_api_version,
        correlation_id,
        client_id: None,
        tagged_fields: None,
    }
}

fn handle_client(mut stream: TcpStream) {
    let request_headers = request_headers(&stream);
    let mut response = Vec::with_capacity(RESPONSE_LENGTH);
    response.put_i32(0); // FIXME: Should be replaced with response message length implementation.
    response.put_i32(request_headers.correlation_id);
    stream.write_all(&response).unwrap();
}

fn main() -> std::io::Result<()> {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
