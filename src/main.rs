use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::str;

// use reqwest;
use std::process::Command;

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
                    /*let params = [("code", s), ("lang", "plain"), ("name", "nc paste")];
                    let client = reqwest::Client::new();
                    let mut res = client.post("https://paste.nikitavbv.com/api/paste")
                        .form(&params)
                        .send().unwrap();
                    let mut buf2: Vec<u8> = vec![];
                    res.copy_to(&mut buf2);*/
                    let output = Command::new("/usr/bin/curl")
                     .arg("-X POST")
                     .arg(format!("--data \"code={}\"", str::replace(s, "\"", "\\\"")))
                     .arg("--data \"lang=plain&name=random\"")
                     .arg("https://paste.nikitavbv.com/api/paste")
                     .output();
                    println!("{:?}", output.unwrap());
                    //stream.write(&buf2).unwrap();
                });
            }
            Err(_) => {
                println!("Error while reading incoming request");
            }
        }
    }
}
