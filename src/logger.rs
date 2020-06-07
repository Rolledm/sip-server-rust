use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
}

#[derive(Debug)]
pub enum Severity {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

fn severity_to_number(severity: Severity) -> u32 {
    match severity {
        Severity::Fatal => 1,
        Severity::Error => 2,
        Severity::Warning => 3,
        Severity::Info => 4,
        Severity::Debug => 5,
    }
}

fn number_to_string(number: u32) -> &'static str {
    match number {
        1 => "FATAL",
        2 => "ERROR",
        3 => "WARNING",
        4 => "INFO",
        5 => "DEBUG",
        _ => ""
    }
}

#[derive(Debug)]
pub struct Logger {
    max_severity: u32,
    file: String,
}

pub fn log(severity: Severity, text: &str) {
    let logger = Logger::get_instance().lock().unwrap();
    match &*logger {
        None => (),
        Some(logger) => logger.log(severity_to_number(severity), text),
    };
}

impl Logger {
    pub fn init(max_severity: Severity, file: String) {
        let mut logger = LOGGER.lock().unwrap();
        if logger.is_none() {
            *logger = Some(Logger {
                max_severity: severity_to_number(max_severity),
                file: file,
            });
        } else {
            panic!("Logger already initialized!")
        }
    }

    pub fn get_instance() -> &'static Mutex<Option<Self>> {
        if LOGGER.lock().unwrap().is_some() {
            &LOGGER
        } else {
            panic!("Logger not initialized!")
        }
    }

    // TODO datetime and output to file
    pub fn log(&self, severity: u32, text: &str) {
        if severity <= self.max_severity {
            println!("{} | {}", number_to_string(severity), text);
        }
    }
}