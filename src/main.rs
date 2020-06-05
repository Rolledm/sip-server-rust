use mongodb::{Client, options::ClientOptions};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::sync::mpsc;

mod http;

fn main() {
    println!("Connecting to DB");
    let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = Client::with_options(client_options).unwrap();
    for db_name in client.list_database_names(None).unwrap() {
        println!("{}", db_name);
    }

    let (tx, rx): (mpsc::Sender<TcpStream>, mpsc::Receiver<TcpStream>) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let l1 = TcpListener::bind("localhost:7878").unwrap();
        for stream in l1.incoming() {
            let stream = stream.unwrap();
            tx1.send(stream).unwrap();
        }
    });
    thread::spawn(move || {
        let l2 = TcpListener::bind("localhost:7879").unwrap();
        for stream in l2.incoming() {
            let stream = stream.unwrap();
            tx.send(stream).unwrap();
        }
    });

    loop {
        on_message_received(rx.recv().unwrap());
    }
}

fn on_message_received(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    if buffer.starts_with(b"GET") || buffer.starts_with(b"POST") {
        println!("HTTP");
        stream.write("http".as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        println!("SIP message received");
        let message = sip_rld::Message::parse(std::str::from_utf8(&buffer).unwrap());
        on_sip_message_received(message, stream);
        //println!("{}", std::str::from_utf8(&buffer).unwrap());
    }
}

fn on_sip_message_received(message: sip_rld::Message, mut stream: TcpStream) {
    match &message.mtype {
        sip_rld::MessageType::Request(request) => {
            match request {
                sip_rld::RequestMethod::Register => on_sip_register_received(message, stream),
                _ => println!("Unknown request!")
            }
        },
        sip_rld::MessageType::Response(responce) => {
            println!("Response: {}", responce)
        }
    }
    //stream.write("200 OK".as_bytes()).unwrap();
    //stream.flush().unwrap();
}

fn on_sip_register_received(message: sip_rld::Message, mut stream: TcpStream) {

}