use std::io::prelude::*;
use std::net::TcpStream;

#[path = "users_collection.rs"] mod users_collection;

pub fn init_users() {
    users_collection::Users::init();
}

pub fn on_sip_message_received(message: sip_rld::Message, stream: TcpStream) {
    match &message.mtype {
        sip_rld::MessageType::Request(request) => {
            match request {
                sip_rld::RequestMethod::Register => on_sip_register_received(message, stream),
                sip_rld::RequestMethod::Invite => on_sip_invite_received(message, stream),
                _ => println!("Unknown request!")
            }
        },
        sip_rld::MessageType::Response(responce) => {
            if responce.starts_with("200") {
                on_sip_ok_received(message, stream);
            } else if responce.starts_with("180") {
                on_sip_ringing_received(message, stream);
            }
        }
    }
}

pub fn on_sip_register_received(mut message: sip_rld::Message, mut stream: TcpStream) {
    println!("{} connected", message.request_uri);
    {
        let mut users = users_collection::Users::get_instance().lock().unwrap();
        match &mut *users {
            None => (),
            Some(users) => {
                message.mtype = sip_rld::MessageType::Response(String::from("200 OK"));
                stream.write(message.build_message().as_bytes()).unwrap();
                stream.flush().unwrap();
                users.users.insert(message.request_uri, stream);
                println!("{:?}", users.users)}
        };
    }
}

pub fn on_sip_invite_received(mut message: sip_rld::Message, mut stream: TcpStream) {
    {
        let mut users = users_collection::Users::get_instance().lock().unwrap();
        match &mut *users {
            None => (),
            Some(users) => {
                match users.users.get(&message.to) {
                    None => (),
                    Some(mut stream) => {
                        stream.write(message.build_message().as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                }
            }
        };
    }
}

pub fn on_sip_ringing_received(mut message: sip_rld::Message, mut stream: TcpStream) {
    {
        let mut users = users_collection::Users::get_instance().lock().unwrap();
        match &mut *users {
            None => (),
            Some(users) => {
                match users.users.get(&message.to) {
                    None => (),
                    Some(mut stream) => {
                        stream.write(message.build_message().as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                }
            }
        };
    }
}

pub fn on_sip_ok_received(mut message: sip_rld::Message, mut stream: TcpStream) {
    {
        let mut users = users_collection::Users::get_instance().lock().unwrap();
        match &mut *users {
            None => (),
            Some(users) => {
                match users.users.get(&message.to) {
                    None => (),
                    Some(mut stream) => {
                        stream.write(message.build_message().as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                }
            }
        };
    }
}