use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::str;

fn main() {
    println!("Paste netcat server");
    start_tcp_server();
}

fn start_tcp_server() {
    let listener = TcpListener::bind("0.0.0.0:4242").unwrap();
    println!("tcp server started");
    for req in listener.incoming() {
        match req {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut content = vec![];
                    loop {
                        let mut buf = [0; 1024];
                        match stream.read(&mut buf) {
                            Ok(n) => {
                                if n == 0 {
                                    // Connection is closed
                                    break;
                                }
                                content.extend_from_slice(&buf[0..n]);
                                if n < buf.len() {
                                    break;
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
                    stream.write(&content).unwrap();
                });
            }
            Err(_) => {
                println!("Error while reading incoming request");
            }
        }
    }
}
