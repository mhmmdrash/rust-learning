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

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 1024;

fn main() {
    let server = TcpListener::bind(LOCAL).unwrap();
    server.set_nonblocking(true).expect("failed to init non blocking");

    let (tx, rx) = mpsc::channel::<String>();
    let mut clients = vec![];
    // let pools = ThreadPool::new(5);

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
        println!("Connected to {}", addr);

        let tx = tx.clone();
        clients.push(socket.try_clone().expect("failed to clone client"));

        // Fetch the message from tcp stream using a seperate thread and send to main thread
        thread::spawn(move || loop {

            let mut buf = vec![0; MSG_SIZE];
            match socket.read_exact(&mut buf) {
                Ok(_) => {
                    let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).unwrap();
                    println!("{}: {:?}", addr, msg);
                    tx.send(msg).unwrap();
                },
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("closing connection with: {}", addr);
                    break;
                }
            }
            thread::sleep(Duration::from_secs(5));
    
        });
    
        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        thread::sleep(Duration::from_secs(5));
        }
    }
}
