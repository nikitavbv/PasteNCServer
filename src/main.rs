use std::io::{Read, Write};
use std::net::TcpListener;
use std::{thread, time, str};

use reqwest;
use serde_json::{Value};

static PASTE_SERVICE_URL: &str = "https://paste.nikitavbv.com";
const PORT: i32 = 4242;
const STREAM_WAIT_TIME: time::Duration = time::Duration::from_millis(400);

fn main() {
    println!("Paste netcat server");
    start_tcp_server();
}

fn start_tcp_server() {
    let listener = match TcpListener::bind(&("0.0.0.0:".to_owned() + &PORT.to_string().to_owned())) {
        Ok(n) => n,
        Err(err) => panic!("Failed to start tcp server: {}", err)
    };
    println!("tcp server started");
    for req in listener.incoming() {
        match req {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut content = Vec::new();
                    let mut waiting_for_next = true;
                    loop {
                        let mut buf = [0; 1024];
                        match stream.read(&mut buf) {
                            Ok(n) => {
                                if n == 0 {
                                    // Connection is closed
                                    if !waiting_for_next {
                                        break;
                                    }

                                    thread::sleep(STREAM_WAIT_TIME);
                                    waiting_for_next = false;
                                } else {
                                    waiting_for_next = true;
                                    content.extend_from_slice(&buf[0..n]);
                                    if n < buf.len() {
                                        break;
                                    }
                                }
                            },
                            Err(err) => {
                                panic!(err);
                            }
                        }
                    }
                    let s = match str::from_utf8(&content) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };
                    println!("New paste code:");
                    println!("{}", s);
                    let params = [("code", s), ("name", "nc paste")];
                    let client = reqwest::Client::new();
                    let mut res = match client.post(&(PASTE_SERVICE_URL.to_owned() + "/api/paste"))
                        .form(&params)
                        .send() {
                            Ok(v) => v,
                            Err(e) => panic!("Failed to make request to paste service: {}", e)
                        };
                    let mut response_buf: Vec<u8> = vec![];
                    res.copy_to(&mut response_buf).expect("No data from paste service");
                    let response_str = match str::from_utf8(&response_buf) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };
                    let response_data: Value = match serde_json::from_str(response_str) {
                        Ok(v) => v,
                        Err(e) => panic!("Failed to parse json: {}", e),
                    };
                    let paste_id = &response_data["id"];
                    let mut paste_addr = "".to_owned();
                    paste_addr.push_str(PASTE_SERVICE_URL);
                    paste_addr.push_str("/");
                    paste_addr.push_str(&paste_id.to_string().replace("\"", "")); 
                    println!("Created paste: {}", paste_addr);
                    stream.write((paste_addr + "\n").as_bytes()).expect("Failed to write response to client");
                });
            }
            Err(_) => {
                println!("Error while reading incoming request");
            }
        }
    }
}
