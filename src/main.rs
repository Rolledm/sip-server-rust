use mongodb::{Client, options::ClientOptions};
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::sync::mpsc;

mod http;
mod sip;
mod logger;
mod users_collection;

fn main() {
    logger::Logger::init(logger::Severity::Debug, "./log.log");
    logger::log(logger::Severity::Info, "Logger initialized.");

    users_collection::Users::init();
    logger::log(logger::Severity::Info, "User collection initialized.");
    
    let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = Client::with_options(client_options).unwrap();
    logger::log(logger::Severity::Info, "Connection to database initialized.");

    let (tx, rx): (mpsc::Sender<TcpStream>, mpsc::Receiver<TcpStream>) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        logger::log(logger::Severity::Info, "HTTP server started.");

        let l1 = TcpListener::bind("localhost:8080").unwrap();
        for stream in l1.incoming() {
            let stream = stream.unwrap();
            tx1.send(stream).unwrap();
        }
    });
    thread::spawn(move || {
        logger::log(logger::Severity::Info, "SIP server started.");
        
        let l2 = TcpListener::bind("localhost:5060").unwrap();
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
    logger::log(logger::Severity::Debug, &format!("Message received: {}", std::str::from_utf8(&buffer).unwrap()));
    if buffer.starts_with(b"GET") || buffer.starts_with(b"POST") {
        http::on_http_message_received(stream);
    } else {
        let message = sip_rld::Message::parse(std::str::from_utf8(&buffer).unwrap());
        sip::on_sip_message_received(message, stream);
    }
}