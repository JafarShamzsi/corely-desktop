use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

pub struct Logger {
    log_file: Option<PathBuf>,
    min_level: LogLevel,
}

impl Logger {
    pub fn new(log_file: Option<PathBuf>, min_level: LogLevel) -> Self {
        Self {
            log_file,
            min_level,
        }
    }
    
    pub fn log(&self, level: LogLevel, message: &str) {
        if !self.should_log(&level) {
            return;
        }
        
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let log_entry = format!("[{}] {}: {}", timestamp, level, message);
        
        // Always print to console
        println!("{}", log_entry);
        
        // Write to file if configured
        if let Some(ref log_file) = self.log_file {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file)
            {
                let _ = writeln!(file, "{}", log_entry);
            }
        }
    }
    
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    
    fn should_log(&self, level: &LogLevel) -> bool {
        match (&self.min_level, level) {
            (LogLevel::Debug, _) => true,
            (LogLevel::Info, LogLevel::Debug) => false,
            (LogLevel::Info, _) => true,
            (LogLevel::Warn, LogLevel::Debug | LogLevel::Info) => false,
            (LogLevel::Warn, _) => true,
            (LogLevel::Error, LogLevel::Error) => true,
            (LogLevel::Error, _) => false,
        }
    }
    
    pub fn set_min_level(&mut self, level: LogLevel) {
        self.min_level = level;
    }
}

// Global logger instance
static mut GLOBAL_LOGGER: Option<Logger> = None;

pub fn init_logger(log_file: Option<PathBuf>, min_level: LogLevel) {
    unsafe {
        GLOBAL_LOGGER = Some(Logger::new(log_file, min_level));
    }
}

pub fn log(level: LogLevel, message: &str) {
    unsafe {
        if let Some(ref logger) = GLOBAL_LOGGER {
            logger.log(level, message);
        }
    }
}

pub fn debug(message: &str) {
    log(LogLevel::Debug, message);
}

pub fn info(message: &str) {
    log(LogLevel::Info, message);
}

pub fn warn(message: &str) {
    log(LogLevel::Warn, message);
}

pub fn error(message: &str) {
    log(LogLevel::Error, message);
}
