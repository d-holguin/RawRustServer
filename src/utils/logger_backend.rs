use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    sync::Mutex,
};

static LOGGER_BACKEND: OnceLock<LoggerBackend> = OnceLock::new();

pub fn init_global_logger(filename: &'static str) {
    if LOGGER_BACKEND.set(LoggerBackend::new(filename).unwrap()).is_err() {
        panic!("Failed to initialize logger");
    }
}

pub fn global_logger() -> &'static LoggerBackend {
    LOGGER_BACKEND.get().expect("Logger not initialized. Call init_global_logger.")
}

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

impl LoggerBackend {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let parent_dir = std::path::Path::new(filename)
            .parent()
            .unwrap_or(&std::path::Path::new("."));

        std::fs::create_dir_all(&parent_dir)?;

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(filename)?;

        Ok(LoggerBackend {
            terminal: Mutex::new(BufWriter::new(std::io::stdout())),
            file: Mutex::new(BufWriter::new(file)),
        })
    }
    pub fn log(&self, level: LogLevel, message: &str) {
        let timestamp = Self::get_timestamp();
        let formatted_message = format!("[{timestamp}] [{}]", message);

        match level {
            LogLevel::INFO => {
                if let Ok(mut terminal) = self.terminal.lock() {
                    let _ = writeln!(terminal, "\x1B[32m[INFO]\x1B[0m {}", formatted_message);
                    let _ = terminal.flush();
                }
            }
            LogLevel::ERROR => {
                if let Ok(mut terminal) = self.terminal.lock() {
                    let _ = writeln!(terminal, "\x1B[31m[ERROR]\x1B[0m {}", formatted_message);
                    let _ = terminal.flush();
                }
            }
        }
        if let Ok(mut file) = self.file.lock() {
            let _ = writeln!(file, "[{}] {}", level.as_ref(), formatted_message);
            let _ = file.flush();
        }
    }
    fn get_timestamp() -> String {
        if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
            let seconds = now.as_secs();
            let millis = now.subsec_millis();
            return format!("{seconds}.{millis:03}");
        }
        "TIMESTAMP_ERROR".to_string()
    }
}
