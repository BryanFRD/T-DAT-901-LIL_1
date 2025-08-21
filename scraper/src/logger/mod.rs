use colored::*;
use chrono::Local;
use crate::models::source::Source;

pub mod macros;

#[derive(Debug)]
pub enum LogType {
    INFO,
    WARN,
    ERROR,
    DEBUG,
}

impl LogType {
    fn as_str(&self) -> ColoredString {
        match self {
            LogType::INFO => "INFO".blue(),
            LogType::WARN => "WARN".yellow(),
            LogType::ERROR => "ERROR".red(),
            LogType::DEBUG => "DEBUG".purple(),
        }
    }
}

pub fn log(log_type: LogType, source: Option<Source>, message: &str) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    if let Some(s) = source {
        println!("{} [{}] [{}] {}", now, log_type.as_str(), s, message);
    } else {
        println!("{} [{}] {}", now, log_type.as_str(), message);
    }
}
