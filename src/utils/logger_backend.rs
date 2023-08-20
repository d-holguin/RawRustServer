use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    sync::Mutex,
};

pub struct LoggerBackend {
    terminal: Mutex<BufWriter<std::io::Stdout>>,
    error_terminal: Mutex<BufWriter<std::io::Stderr>>,
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
        let parent_dir = std::path::Path::new(filename)
            .parent()
            .unwrap_or(&std::path::Path::new("."));

        if let Err(e) = std::fs::create_dir_all(&parent_dir) {
            panic!(
                "Failed to create log directory: {}. Error: {}",
                parent_dir.display(),
                e
            );
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(filename)
            .expect("Could not create logger backend");
        let file = BufWriter::new(file);
        let terminal = BufWriter::new(std::io::stdout());
        let error_terminal = BufWriter::new(std::io::stderr());
        LoggerBackend {
            terminal: Mutex::new(terminal),
            file: Mutex::new(file),
            error_terminal: Mutex::new(error_terminal),
        }
    }
    pub fn log(&self, level: LogLevel, message: &str) {
        match level {
            LogLevel::INFO => {
                if let Ok(mut terminal) = self.terminal.lock() {
                    let _ = writeln!(terminal, "\x1B[32m[INFO]\x1B[0m {}", message);
                    let _ = terminal.flush();
                }
            }
            LogLevel::ERROR => {
                if let Ok(mut error_terminal) = self.error_terminal.lock() {
                    let _ = writeln!(error_terminal, "\x1B[31m[ERROR]\x1B[0m {}", message);
                    let _ = error_terminal.flush();
                }
            }
        }
        if let Ok(mut file) = self.file.lock() {
            let _ = writeln!(file, "[{}] {}", level.as_ref(), message);
            let _ = file.flush();
        } else {
            eprint!("error writing to file");
        }
    }
}
