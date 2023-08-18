use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    sync::Mutex,
};

pub struct LoggerBackend {
    terminal: Mutex<BufWriter<std::io::Stdout>>,
    file: Mutex<BufWriter<File>>,
}

#[derive(Debug)]
pub enum LogLevel {
    INFO,
    ERROR,
}

impl AsRef<str> for LogLevel {
    fn as_ref(&self) -> &str {
        match *self {
            LogLevel::INFO => "INFO",
            LogLevel::ERROR => "ERROR",
        }
    }
}

static INIT: std::sync::Once = std::sync::Once::new();
static mut LOGGER_BACKEND: Option<LoggerBackend> = None;
pub fn init_global_logger(filename: &'static str) {
    INIT.call_once(|| unsafe {
        LOGGER_BACKEND = Some(LoggerBackend::new(filename));
    });
}

pub fn global_logger() -> &'static LoggerBackend {
    unsafe {
        if LOGGER_BACKEND.is_none() {
            panic!("Logger has not been initialized. Call init_global_logger first.");
        }
        LOGGER_BACKEND.as_ref().unwrap()
    }
}

impl LoggerBackend {
    pub fn new(filename: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(filename)
            .expect("Could create logger backend");
        let file = BufWriter::new(file);
        let terminal = BufWriter::new(std::io::stdout());
        LoggerBackend {
            terminal: Mutex::new(terminal),
            file: Mutex::new(file),
        }
    }
    pub fn log(&self, level: LogLevel, message: &str) {
        let mut terminal = self.terminal.lock().unwrap();
        let mut file = self.file.lock().unwrap();

        match level {
            LogLevel::INFO => {
                writeln!(terminal, "\x1B[32m[INFO]\x1B[0m {}", message).unwrap();
                writeln!(file, "[INFO] {}", message).unwrap();
            }
            LogLevel::ERROR => {
                writeln!(terminal, "\x1B[31m[ERROR]\x1B[0m {}", message).unwrap();
                writeln!(file, "[ERROR] {}", message).unwrap();
            }
        }

        terminal.flush().unwrap();
        file.flush().unwrap();
    }
}
