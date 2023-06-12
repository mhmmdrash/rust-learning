// use std::net::TcpListener;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
// use std::fs;
// use std::io::BufReader;
use std::time::Duration;
// use threadpool::ThreadPool;

const LOCAL: &str = "127.0.0.1:6000";

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to set up non blocking");

    let (tx, rx) = mpsc::channel::<String>();
    
    thread::spawn(move || loop {
        let mut buf = vec![0; 1024];
        match client.read_exact(&mut buf) {
            Ok(_) => {
                let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("recieved: {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Error Occured at match client buf!");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buf = msg.clone().into_bytes();
                buf.resize(1024, 0);
                client.write_all(&buf).expect("write to socket failed");
                println!("Message sent {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(_) => {
                println!("Error Occured at match rx!");
                break;
            }
        }
        thread::sleep(Duration::from_secs(5));
    });

    println!("Type message here: ");
    loop {
        let mut buff = String::new();
        std::io::stdout().flush();
        std::io::stdin().read_line(&mut buff).expect("Argument read failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }
    println!("Bye Bye !");

}