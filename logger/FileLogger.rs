use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

pub trait Logger {
    fn log(&self, message: &str);
}

pub struct StreamLogger {
    log_level: Level,
}

impl StreamLogger {
    pub fn new(level: Level) -> Self {
        StreamLogger { log_level: level }
    }

    pub fn set_log_level(&mut self, level: Level) {
        self.log_level = level;
    }
}

impl Logger for StreamLogger {
    fn log(&self, message: &str) {
        println!("{}", message);
    }
}

pub struct FileLogger {
    stream_logger: StreamLogger,
    file: Option<File>,
}

impl FileLogger {
    pub fn new(level: Level) -> Self {
        FileLogger {
            stream_logger: StreamLogger::new(level),
            file: None,
        }
    }

    pub fn init(&mut self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        self.file = Some(file);
        Ok(())
    }

    pub fn log_to_file(&self, message: &str) -> io::Result<()> {
        if let Some(ref file) = self.file {
            writeln!(file, "{}", message)?;
        }
        Ok(())
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        if let Some(_) = &self.file {
            self.log_to_file(message).unwrap();
        } else {
            println!("{}", message); 
        }
    }
}

fn main() {
    let mut file_logger = FileLogger::new(Level::Info);
    file_logger.init("log.txt").unwrap();
    file_logger.log("This is a log message!");
}
