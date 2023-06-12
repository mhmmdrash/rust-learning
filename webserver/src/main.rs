use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::fs;
use std::io::BufReader;
use std::time::Duration;
use threadpool::ThreadPool;

const LOCAL: &str = "127.0.0.1:7878";
// const MSG_SIZE: usize = 1024;

fn main() {
    let server = TcpListener::bind(LOCAL).unwrap();

    let pools = ThreadPool::new(5);

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Connected to {}", addr);
            pools.execute(move || {
                handle_connection(socket);
            });
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /about HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "about.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    // let mut buf: [u8; 1024] = [0; MSG_SIZE];
    // stream.read(&mut buf).unwrap();
    // let home: &[u8] = b"GET / HTTP/1.1\r\n";
    // let about: &[u8] = b"GET /about HTTP/1.1\r\n";
    // let (status_line, filename) = match buf {
    //     home =>  ("HTTP/1.1 200 OK", "index.html"),
    //     about => ("HTTP/1.1 200 OK", "about.html"),
    //     _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };
    // let (status_line, filename) = 
    //     if buf.starts_with(home) {
    //         ("HTTP/1.1 200 OK", "index.html")
    //     } else if buf.starts_with(about) {
    //         ("HTTP/1.1 200 OK", "about.html")
    //     } else {
    //         ("HTTP/1.1 404 NOT FOUND", "404.html")
    //     };
    let contents: String = fs::read_to_string(filename).unwrap();
    let response: String = format!(
        "{}\r\nContent-Length {}\r\n\r\n{}",
        status_line,
        contents.len(), 
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

