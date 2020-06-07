use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Mutex;

lazy_static! {
    static ref USERS: Mutex<Option<Users>> = Mutex::new(None);
}


#[derive(Debug)]
pub struct Users {
    pub users: HashMap<String, TcpStream>,
}

impl Users {
    pub fn init() {
        let mut users = USERS.lock().unwrap();
        if users.is_none() {
            *users = Some(Users {
                users: HashMap::new(),
            });
        } else {
            panic!("Users collection already initialized!")
        }
    }

    pub fn get_instance() -> &'static Mutex<Option<Self>> {
        if USERS.lock().unwrap().is_some() {
            &USERS
        } else {
            panic!("Users collection not initialized!")
        }
    }
}