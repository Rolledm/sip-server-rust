use mongodb::{Client, options::ClientOptions};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::sync::mpsc;

mod sip;
mod http;

enum Message {
    SIP(String),
    HTTP(String),
}

fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = Client::with_options(client_options).unwrap();
    for db_name in client.list_database_names(None).unwrap() {
        println!("{}", db_name);
    }

    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let l1 = TcpListener::bind("localhost:7878").unwrap();
        for stream in l1.incoming() {
            let stream = stream.unwrap();

            http::handle_connection(stream);

            tx1.send(1).unwrap();
        }
    });
    thread::spawn(move || {
        let l2 = TcpListener::bind("localhost:7879").unwrap();
        for stream in l2.incoming() {
            tx.send(2).unwrap();
        }
    });

    loop {
        println!("{:?}", rx.recv().unwrap());
    }
}
