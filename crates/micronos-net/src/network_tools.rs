use alloc::string::{String, ToString};
use alloc::vec::Vec;
use micronos_core::types::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
}

impl Protocol {
    pub fn name(&self) -> &'static str {
        match self {
            Protocol::TCP => "TCP",
            Protocol::UDP => "UDP",
            Protocol::ICMP => "ICMP",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PingResult {
    pub node_id: NodeId,
    pub seq: u32,
    pub ttl: u8,
    pub latency_ms: u64,
    pub success: bool,
    pub error: Option<String>,
}

impl PingResult {
    pub fn success(node_id: NodeId, seq: u32, latency_ms: u64) -> Self {
        PingResult {
            node_id,
            seq,
            ttl: 64,
            latency_ms,
            success: true,
            error: None,
        }
    }

    pub fn failure(node_id: NodeId, seq: u32, error: &str) -> Self {
        PingResult {
            node_id,
            seq,
            ttl: 0,
            latency_ms: 0,
            success: false,
            error: Some(error.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkTest {
    pub name: String,
    pub protocol: Protocol,
    pub source_port: u16,
    pub dest_port: u16,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub packets_lost: u64,
}

impl NetworkTest {
    pub fn new(name: &str, protocol: Protocol) -> Self {
        NetworkTest {
            name: name.to_string(),
            protocol,
            source_port: 0,
            dest_port: 0,
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            packets_lost: 0,
        }
    }

    pub fn send_packet(&mut self, size: usize) {
        self.packets_sent += 1;
        self.bytes_sent += size as u64;
    }

    pub fn receive_packet(&mut self, size: usize) {
        self.packets_received += 1;
        self.bytes_received += size as u64;
    }

    pub fn packet_loss_percent(&self) -> f64 {
        if self.packets_sent == 0 {
            0.0
        } else {
            let lost = self.packets_sent - self.packets_received;
            (lost as f64 / self.packets_sent as f64) * 100.0
        }
    }
}

pub struct NetworkTools {
    ping_history: Vec<PingResult>,
    tests: Vec<NetworkTest>,
}

impl NetworkTools {
    pub fn new() -> Self {
        NetworkTools {
            ping_history: Vec::new(),
            tests: Vec::new(),
        }
    }

    pub fn ping(&mut self, node_id: NodeId, seq: u32) -> PingResult {
        let base_latency: u64 = 10 + (seq % 50) as u64;
        let jitter: u64 = if seq % 5 == 0 { 100 } else { 0 };

        let result = if jitter == 0 {
            PingResult::success(node_id, seq, base_latency)
        } else {
            PingResult::failure(node_id, seq, "Request timeout")
        };

        self.ping_history.push(result.clone());
        if self.ping_history.len() > 100 {
            self.ping_history.remove(0);
        }

        result
    }

    pub fn ping_with_ttl(&mut self, node_id: NodeId, seq: u32, ttl: u8) -> PingResult {
        let result = if ttl >= 64 {
            PingResult::failure(node_id, seq, "TTL exceeded")
        } else {
            let latency = 10 + (ttl * 5) as u64 + (seq % 20) as u64;
            PingResult {
                node_id,
                seq,
                ttl,
                latency_ms: latency,
                success: true,
                error: None,
            }
        };

        self.ping_history.push(result.clone());
        result
    }

    pub fn get_ping_stats(&self) -> PingStats {
        let total = self.ping_history.len();
        let successful = self.ping_history.iter().filter(|p| p.success).count();
        let failed = total - successful;

        let latencies: Vec<u64> = self
            .ping_history
            .iter()
            .filter(|p| p.success)
            .map(|p| p.latency_ms)
            .collect();

        let min = latencies.iter().min().copied().unwrap_or(0);
        let max = latencies.iter().max().copied().unwrap_or(0);
        let avg = if latencies.is_empty() {
            0.0
        } else {
            latencies.iter().sum::<u64>() as f64 / latencies.len() as f64
        };

        PingStats {
            total,
            successful,
            failed,
            min_latency: min,
            max_latency: max,
            avg_latency: avg,
        }
    }

    pub fn clear_ping_history(&mut self) {
        self.ping_history.clear();
    }

    pub fn start_test(&mut self, name: &str, protocol: Protocol) -> usize {
        let test = NetworkTest::new(name, protocol);
        self.tests.push(test);
        self.tests.len() - 1
    }

    pub fn get_test(&mut self, id: usize) -> Option<&mut NetworkTest> {
        self.tests.get_mut(id)
    }

    pub fn list_tests(&self) -> &[NetworkTest] {
        &self.tests
    }

    pub fn format_ping_stats(&self) -> String {
        let stats = self.get_ping_stats();

        let mut output =
            String::from("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                    Ping Statistics                         ║\n");
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");
        output.push_str(&alloc::format!(
            "║  Packets: Sent = {:>5}, Received = {:>5}, Lost = {:>5} ({:>5.1}%)  ║\n",
            stats.total,
            stats.successful,
            stats.failed,
            stats.failed as f64 / stats.total.max(1) as f64 * 100.0
        ));
        output.push_str("║  Round Trip Times:                                             ║\n");
        output.push_str(&alloc::format!(
            "║    Minimum = {:>4} ms, Maximum = {:>4} ms, Average = {:>6.2} ms      ║\n",
            stats.min_latency,
            stats.max_latency,
            stats.avg_latency
        ));
        output.push_str("╚════════════════════════════════════════════════════════════════╝");
        output
    }
}

#[derive(Debug, Clone)]
pub struct PingStats {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub min_latency: u64,
    pub max_latency: u64,
    pub avg_latency: f64,
}

impl Default for NetworkTools {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping() {
        let mut tools = NetworkTools::new();
        let node = NodeId::default();
        let result = tools.ping(node, 1);
        assert!(result.success);
    }

    #[test]
    fn test_ping_stats() {
        let mut tools = NetworkTools::new();
        let node = NodeId::default();
        tools.ping(node, 1);
        tools.ping(node, 2);
        tools.ping(node, 3);

        let stats = tools.get_ping_stats();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.successful, 3);
    }

    #[test]
    fn test_network_test() {
        let mut test = NetworkTest::new("test", Protocol::TCP);
        test.send_packet(100);
        test.receive_packet(100);
        assert_eq!(test.packets_sent, 1);
        assert_eq!(test.packets_received, 1);
    }
}
