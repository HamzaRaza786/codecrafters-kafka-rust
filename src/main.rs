#![allow(unused_imports, dead_code)]
use bytes::{Buf, BufMut};
use std::{
    any, fs,
    io::{prelude::*, BufReader, Cursor, Read, Seek, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    string,
};

const MESSAGE_LENGTH_SIZE: usize = 4;

enum ErrorCodes {
    UnsupportedVersion = 35,
}

struct Headers {
    request_api_key: i16,
    request_api_version: i16,
    correlation_id: i32,
    client_id: Option<String>,
    tagged_fields: Option<String>,
}
struct Body {
    correlation_id: i32,
    error_code: i16,
    num_api_keys_records: i8,
    api_key: i16,
    min_version: i16,
    max_version: i16,
    tag_buffer: i8,
    throttle_time_ms: i32,
    tag_buffer_length: i8,
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
fn construct_response(request_headers: &Headers) -> Body {
    let supported = (0..5).contains(&request_headers.request_api_version);

    let error_code = match supported {
        true => 0,
        false => ErrorCodes::UnsupportedVersion as i16,
    };

    return Body {
        correlation_id: request_headers.correlation_id,
        error_code,
        num_api_keys_records: 2,
        api_key: request_headers.request_api_key,
        min_version: 0,
        max_version: 4,
        tag_buffer: 0,
        throttle_time_ms: 420,
        tag_buffer_length: 0,
    };
}

fn handle_client(mut stream: TcpStream) {
    let request_headers = request_headers(&stream);
    let body = construct_response(&request_headers);

    let mut response_body = vec![];
    response_body.put_i32(body.correlation_id);
    response_body.put_i16(body.error_code);
    response_body.put_i8(body.num_api_keys_records);
    response_body.put_i16(body.api_key);
    response_body.put_i16(body.min_version);
    response_body.put_i16(body.max_version);
    response_body.put_i8(body.tag_buffer);
    response_body.put_i32(body.throttle_time_ms);
    response_body.put_i8(body.tag_buffer_length);

    let response_message_length = response_body.len() as usize;
    let mut response = Vec::with_capacity(MESSAGE_LENGTH_SIZE + response_message_length);
    response.put_i32(response_message_length as i32);
    response.extend(response_body);
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
