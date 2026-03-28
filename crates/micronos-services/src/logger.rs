use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

static LOG_LEVEL: AtomicU64 = AtomicU64::new(3);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

impl LogLevel {
    pub fn from_u64(val: u64) -> Self {
        match val {
            0 => LogLevel::Error,
            1 => LogLevel::Warn,
            2 => LogLevel::Info,
            3 => LogLevel::Debug,
            _ => LogLevel::Trace,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp_ms: u64,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
}

pub struct Logger {
    entries: Vec<LogEntry>,
    max_entries: usize,
    silent: bool,
}

impl Logger {
    pub const MAX_ENTRIES: usize = 1000;

    pub fn new() -> Self {
        Logger {
            entries: Vec::new(),
            max_entries: Self::MAX_ENTRIES,
            silent: false,
        }
    }

    pub fn set_level(level: LogLevel) {
        LOG_LEVEL.store(level as u64, Ordering::Relaxed);
    }

    pub fn get_level() -> LogLevel {
        LogLevel::from_u64(LOG_LEVEL.load(Ordering::Relaxed))
    }

    pub fn set_silent(&mut self, silent: bool) {
        self.silent = silent;
    }

    pub fn log(&mut self, level: LogLevel, source: &str, message: &str) {
        let current_level = Self::get_level();
        if level as u64 > current_level as u64 {
            return;
        }

        let timestamp = self.timestamp();
        let entry = LogEntry {
            timestamp_ms: timestamp,
            level,
            source: source.to_string(),
            message: message.to_string(),
        };

        if !self.silent {
            self.print_entry(&entry);
        }

        self.entries.push(entry);

        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    fn timestamp(&self) -> u64 {
        self.entries.len() as u64 * 100
    }

    fn print_entry(&self, entry: &LogEntry) {
        let level_str = match entry.level {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN ",
            LogLevel::Info => "INFO ",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        };
        let _ = level_str;
    }

    pub fn error(&mut self, source: &str, message: &str) {
        self.log(LogLevel::Error, source, message);
    }

    pub fn warn(&mut self, source: &str, message: &str) {
        self.log(LogLevel::Warn, source, message);
    }

    pub fn info(&mut self, source: &str, message: &str) {
        self.log(LogLevel::Info, source, message);
    }

    pub fn debug(&mut self, source: &str, message: &str) {
        self.log(LogLevel::Debug, source, message);
    }

    pub fn trace(&mut self, source: &str, message: &str) {
        self.log(LogLevel::Trace, source, message);
    }

    pub fn get_entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn entries_count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    pub fn format_entries(&self) -> String {
        let mut output =
            String::from("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                      System Log                            ║\n");
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");
        for entry in &self.entries {
            let level_str = match entry.level {
                LogLevel::Error => "ERROR",
                LogLevel::Warn => "WARN ",
                LogLevel::Info => "INFO ",
                LogLevel::Debug => "DEBUG",
                LogLevel::Trace => "TRACE",
            };
            let msg = if entry.message.len() > 40 {
                alloc::format!("{}...", &entry.message[..37])
            } else {
                entry.message.clone()
            };
            output.push_str(&alloc::format!(
                "║ [{}] {} | {:8} | {:40} ║\n",
                entry.timestamp_ms,
                level_str,
                entry.source,
                msg
            ));
        }
        output.push_str("╚════════════════════════════════════════════════════════════════╝");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_creation() {
        let logger = Logger::new();
        assert_eq!(logger.entries_count(), 0);
    }

    #[test]
    fn test_log_levels() {
        let mut logger = Logger::new();
        logger.set_silent(true);
        logger.error("test", "error message");
        assert_eq!(logger.entries_count(), 1);
    }

    #[test]
    fn test_log_level_filtering() {
        let mut logger = Logger::new();
        logger.set_silent(true);
        Logger::set_level(LogLevel::Error);
        logger.info("test", "info message");
        assert_eq!(logger.entries_count(), 0);
        logger.error("test", "error message");
        assert_eq!(logger.entries_count(), 1);
        Logger::set_level(LogLevel::Debug);
    }

    #[test]
    fn test_clear_entries() {
        let mut logger = Logger::new();
        logger.set_silent(true);
        logger.info("test", "message");
        assert_eq!(logger.entries_count(), 1);
        logger.clear();
        assert_eq!(logger.entries_count(), 0);
    }
}
