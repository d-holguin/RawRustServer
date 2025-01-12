use crate::utils::{logger_backend, LogLevel};

pub fn info(message: &str) {
    logger_backend::global_logger().log(LogLevel::INFO, message);
}

pub fn error(message: &str) {
    logger_backend::global_logger().log(LogLevel::ERROR, message);
}
