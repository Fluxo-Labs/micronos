use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct StatEntry {
    pub timestamp_ms: u64,
    pub name: String,
    pub value: f64,
    pub unit: String,
}

impl StatEntry {
    pub fn new(timestamp_ms: u64, name: &str, value: f64, unit: &str) -> Self {
        StatEntry {
            timestamp_ms,
            name: name.to_string(),
            value,
            unit: unit.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu_usage: f64,
    pub memory_used: u64,
    pub memory_total: u64,
    pub storage_used: u64,
    pub storage_total: u64,
    pub process_count: u32,
    pub network_packets_in: u64,
    pub network_packets_out: u64,
    pub uptime_ms: u64,
}

impl Default for SystemStats {
    fn default() -> Self {
        SystemStats {
            cpu_usage: 0.0,
            memory_used: 0,
            memory_total: 0,
            storage_used: 0,
            storage_total: 0,
            process_count: 0,
            network_packets_in: 0,
            network_packets_out: 0,
            uptime_ms: 0,
        }
    }
}

pub struct StatsCollector {
    history: Vec<StatEntry>,
    max_history: usize,
    current_stats: SystemStats,
    tick_count: u64,
}

impl StatsCollector {
    pub fn new() -> Self {
        StatsCollector {
            history: Vec::new(),
            max_history: 100,
            current_stats: SystemStats::default(),
            tick_count: 0,
        }
    }

    pub fn set_max_history(&mut self, max: usize) {
        self.max_history = max;
        while self.history.len() > max {
            self.history.remove(0);
        }
    }

    pub fn record_stat(&mut self, name: &str, value: f64, unit: &str) {
        let entry = StatEntry::new(self.tick_count * 100, name, value, unit);
        self.history.push(entry);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    pub fn update_stats(&mut self, stats: SystemStats) {
        let cpu_usage = stats.cpu_usage;
        let mem_used = stats.memory_used;
        let mem_total = stats.memory_total;
        let stor_used = stats.storage_used;
        let stor_total = stats.storage_total;
        let proc_count = stats.process_count;
        let net_in = stats.network_packets_in;
        let net_out = stats.network_packets_out;

        self.current_stats = stats;
        self.tick_count += 1;

        self.record_stat("cpu_usage", cpu_usage, "%");
        self.record_stat(
            "memory_usage",
            if mem_total > 0 {
                (mem_used as f64 / mem_total as f64) * 100.0
            } else {
                0.0
            },
            "%",
        );
        self.record_stat(
            "storage_usage",
            if stor_total > 0 {
                (stor_used as f64 / stor_total as f64) * 100.0
            } else {
                0.0
            },
            "%",
        );
        self.record_stat("process_count", proc_count as f64, "count");
        self.record_stat("network_in", net_in as f64, "packets");
        self.record_stat("network_out", net_out as f64, "packets");
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        self.current_stats.uptime_ms = self.tick_count * 100;
    }

    pub fn get_current_stats(&self) -> &SystemStats {
        &self.current_stats
    }

    pub fn get_history(&self) -> &[StatEntry] {
        &self.history
    }

    pub fn get_history_for(&self, name: &str) -> Vec<&StatEntry> {
        self.history.iter().filter(|e| e.name == name).collect()
    }

    pub fn get_average(&self, name: &str) -> Option<f64> {
        let entries: Vec<&StatEntry> = self.get_history_for(name);
        if entries.is_empty() {
            return None;
        }
        let sum: f64 = entries.iter().map(|e| e.value).sum();
        Some(sum / entries.len() as f64)
    }

    pub fn get_min(&self, name: &str) -> Option<f64> {
        self.get_history_for(name)
            .iter()
            .map(|e| e.value)
            .fold(None, |acc, x| {
                Some(acc.map_or(x, |a| if x < a { x } else { a }))
            })
    }

    pub fn get_max(&self, name: &str) -> Option<f64> {
        self.get_history_for(name)
            .iter()
            .map(|e| e.value)
            .fold(None, |acc, x| {
                Some(acc.map_or(x, |a| if x > a { x } else { a }))
            })
    }

    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn format_current_stats(&self) -> String {
        let stats = &self.current_stats;
        let mem_used_mb = stats.memory_used as f64 / (1024.0 * 1024.0);
        let mem_total_mb = stats.memory_total as f64 / (1024.0 * 1024.0);
        let storage_used_mb = stats.storage_used as f64 / (1024.0 * 1024.0);
        let storage_total_mb = stats.storage_total as f64 / (1024.0 * 1024.0);

        let mut output =
            String::from("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                   System Statistics                       ║\n");
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");
        output.push_str(&alloc::format!(
            "║  CPU Usage:      {:>6.1}%                                      ║\n",
            stats.cpu_usage
        ));
        output.push_str(&alloc::format!(
            "║  Memory:         {:>6.1}/{:<6.1} MB                          ║\n",
            mem_used_mb,
            mem_total_mb
        ));
        output.push_str(&alloc::format!(
            "║  Storage:        {:>6.1}/{:<6.1} MB                          ║\n",
            storage_used_mb,
            storage_total_mb
        ));
        output.push_str(&alloc::format!(
            "║  Processes:      {:>6}                                        ║\n",
            stats.process_count
        ));
        output.push_str(&alloc::format!(
            "║  Network In:     {:>6} packets                               ║\n",
            stats.network_packets_in
        ));
        output.push_str(&alloc::format!(
            "║  Network Out:    {:>6} packets                               ║\n",
            stats.network_packets_out
        ));
        output.push_str(&alloc::format!(
            "║  Uptime:         {:>6} ms                                    ║\n",
            stats.uptime_ms
        ));
        output.push_str(&alloc::format!(
            "║  History:        {:>6} entries                               ║\n",
            self.history.len()
        ));
        output.push_str("╚════════════════════════════════════════════════════════════════╝");
        output
    }
}

impl Default for StatsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_collector() {
        let mut collector = StatsCollector::new();
        collector.record_stat("test", 42.0, "units");
        assert_eq!(collector.history_count(), 1);
    }

    #[test]
    fn test_average() {
        let mut collector = StatsCollector::new();
        collector.record_stat("cpu", 10.0, "%");
        collector.record_stat("cpu", 20.0, "%");
        collector.record_stat("cpu", 30.0, "%");
        assert_eq!(collector.get_average("cpu"), Some(20.0));
    }

    #[test]
    fn test_min_max() {
        let mut collector = StatsCollector::new();
        collector.record_stat("val", 5.0, "");
        collector.record_stat("val", 10.0, "");
        collector.record_stat("val", 15.0, "");
        assert_eq!(collector.get_min("val"), Some(5.0));
        assert_eq!(collector.get_max("val"), Some(15.0));
    }
}
