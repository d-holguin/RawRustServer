use super::logger_backend::LogLevel;

pub fn info(message: &str) {
    super::logger_backend::global_logger().log(LogLevel::INFO, message);
}

pub fn error(message: &str) {
    super::logger_backend::global_logger().log(LogLevel::ERROR, message);
}
