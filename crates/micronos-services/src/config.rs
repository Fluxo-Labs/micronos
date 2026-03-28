use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Float(f64),
    List(Vec<String>),
}

impl ConfigValue {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            ConfigValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ConfigValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ConfigValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

pub struct Config {
    data: BTreeMap<String, ConfigValue>,
    modified: bool,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config {
            data: BTreeMap::new(),
            modified: false,
        };
        config.load_defaults();
        config
    }

    fn load_defaults(&mut self) {
        self.set_string("system.hostname", "micronos");
        self.set_string("system.version", "0.1.0");
        self.set_integer("system.max_processes", 1024);
        self.set_integer("system.max_memory", 1_000_000_000);
        self.set_bool("system.auto_boot", true);
        self.set_string("network.default_protocol", "p2p");
        self.set_integer("network.max_peers", 256);
        self.set_integer("network.scan_interval_ms", 5000);
        self.set_bool("network.discovery_enabled", true);
        self.set_string("storage.filesystem", "vfs");
        self.set_integer("storage.block_size", 4096);
        self.set_integer("logger.max_entries", 1000);
        self.set_string("logger.level", "info");
        self.set_bool("logger.console_output", true);
        self.set_integer("health.check_interval_ms", 1000);
        self.set_integer("health.critical_threshold", 30);
        self.set_integer("health.recovery_timeout_ms", 5000);
    }

    pub fn get(&self, key: &str) -> Option<&ConfigValue> {
        self.data.get(key)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|v| v.as_string().cloned())
    }

    pub fn get_integer(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| v.as_integer())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    pub fn set(&mut self, key: &str, value: ConfigValue) {
        self.data.insert(key.to_string(), value);
        self.modified = true;
    }

    pub fn set_string(&mut self, key: &str, value: &str) {
        self.set(key, ConfigValue::String(value.to_string()));
    }

    pub fn set_integer(&mut self, key: &str, value: i64) {
        self.set(key, ConfigValue::Integer(value));
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.set(key, ConfigValue::Boolean(value));
    }

    pub fn remove(&mut self, key: &str) -> Option<ConfigValue> {
        self.modified = true;
        self.data.remove(key)
    }

    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn reset_modified(&mut self) {
        self.modified = false;
    }

    pub fn export(&self) -> String {
        let mut output = String::new();
        output.push_str("# MicronOS Configuration\n");
        output.push_str("# Auto-generated configuration file\n\n");

        for (key, value) in &self.data {
            let value_str = match value {
                ConfigValue::String(s) => format!("\"{}\"", s),
                ConfigValue::Integer(i) => i.to_string(),
                ConfigValue::Boolean(b) => b.to_string(),
                ConfigValue::Float(f) => f.to_string(),
                ConfigValue::List(l) => format!("[{}]", l.join(", ")),
            };
            output.push_str(&format!("{} = {}\n", key, value_str));
        }
        output
    }

    pub fn format_all(&self) -> String {
        let mut output =
            String::from("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                   System Configuration                    ║\n");
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");

        let mut current_section = String::new();
        for (key, value) in &self.data {
            let section = key.split('.').next().unwrap_or("");
            if section != current_section {
                current_section = section.to_string();
                output.push_str(
                    "╠════════════════════════════════════════════════════════════════╣\n",
                );
                output.push_str(&alloc::format!(
                    "║ [{}]                                                        ║\n",
                    section.to_uppercase()
                ));
            }

            let value_str = match value {
                ConfigValue::String(s) => {
                    if s.len() > 35 {
                        format!("{}...", &s[..32])
                    } else {
                        s.clone()
                    }
                }
                ConfigValue::Integer(i) => i.to_string(),
                ConfigValue::Boolean(b) => b.to_string(),
                ConfigValue::Float(f) => format!("{:.2}", f),
                ConfigValue::List(l) => format!("[{} items]", l.len()),
            };

            let key_short = key.split('.').next_back().unwrap_or(key);
            output.push_str(&alloc::format!(
                "║  {:20} │ {:35} ║\n",
                key_short,
                value_str
            ));
        }
        output.push_str("╚════════════════════════════════════════════════════════════════╝");
        output
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new();
        assert!(config.get_string("system.hostname").is_some());
    }

    #[test]
    fn test_set_get() {
        let mut config = Config::new();
        config.set_string("test.key", "value");
        assert_eq!(config.get_string("test.key"), Some("value".to_string()));
    }

    #[test]
    fn test_integer() {
        let mut config = Config::new();
        config.set_integer("test.count", 42);
        assert_eq!(config.get_integer("test.count"), Some(42));
    }

    #[test]
    fn test_bool() {
        let mut config = Config::new();
        config.set_bool("test.flag", true);
        assert_eq!(config.get_bool("test.flag"), Some(true));
    }

    #[test]
    fn test_remove() {
        let mut config = Config::new();
        config.set_string("test.key", "value");
        assert!(config.remove("test.key").is_some());
        assert!(config.get("test.key").is_none());
    }
}
