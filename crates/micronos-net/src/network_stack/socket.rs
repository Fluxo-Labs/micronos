use alloc::vec::Vec;

use crate::network_stack::protocol::{IpEndpoint, Protocol, SocketState, SocketType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SocketId(pub u32);

impl SocketId {
    pub fn new(id: u32) -> Self {
        SocketId(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct TcpSocket {
    pub id: SocketId,
    pub socket_type: SocketType,
    pub local_addr: [u8; 4],
    pub local_port: u16,
    pub remote_addr: [u8; 4],
    pub remote_port: u16,
    pub state: SocketState,
    pub receive_buffer: Vec<u8>,
    pub send_buffer: Vec<u8>,
}

impl TcpSocket {
    pub fn new(id: SocketId, local: IpEndpoint, remote: IpEndpoint) -> Self {
        let local_bytes = local.addr.octets();
        let remote_bytes = remote.addr.octets();

        TcpSocket {
            id,
            socket_type: SocketType::Stream,
            local_addr: local_bytes,
            local_port: local.port.value(),
            remote_addr: remote_bytes,
            remote_port: remote.port.value(),
            state: SocketState::Closed,
            receive_buffer: Vec::new(),
            send_buffer: Vec::new(),
        }
    }

    pub fn is_bound(&self) -> bool {
        self.state != SocketState::Closed
    }

    pub fn is_connected(&self) -> bool {
        self.state == SocketState::Established
    }
}

#[derive(Debug, Clone)]
pub struct UdpSocket {
    pub id: SocketId,
    pub socket_type: SocketType,
    pub local_addr: [u8; 4],
    pub local_port: u16,
    pub remote_addr: Option<[u8; 4]>,
    pub remote_port: u16,
}

impl UdpSocket {
    pub fn new(id: SocketId, local: IpEndpoint) -> Self {
        let local_bytes = local.addr.octets();

        UdpSocket {
            id,
            socket_type: SocketType::Datagram,
            local_addr: local_bytes,
            local_port: local.port.value(),
            remote_addr: None,
            remote_port: 0,
        }
    }

    pub fn set_remote(&mut self, addr: [u8; 4], port: u16) {
        self.remote_addr = Some(addr);
        self.remote_port = port;
    }

    pub fn is_bound(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketError {
    WouldBlock,
    ConnectionRefused,
    ConnectionReset,
    ConnectionTimeout,
    NotConnected,
    InvalidArgument,
    TooManySockets,
    AddressInUse,
    AddressNotAvailable,
}

impl core::fmt::Display for SocketError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SocketError::WouldBlock => write!(f, "Operation would block"),
            SocketError::ConnectionRefused => write!(f, "Connection refused"),
            SocketError::ConnectionReset => write!(f, "Connection reset"),
            SocketError::ConnectionTimeout => write!(f, "Connection timeout"),
            SocketError::NotConnected => write!(f, "Not connected"),
            SocketError::InvalidArgument => write!(f, "Invalid argument"),
            SocketError::TooManySockets => write!(f, "Too many sockets"),
            SocketError::AddressInUse => write!(f, "Address in use"),
            SocketError::AddressNotAvailable => write!(f, "Address not available"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SocketStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors: u64,
    pub dropped: u64,
}

impl SocketStats {
    pub fn new() -> Self {
        SocketStats {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            errors: 0,
            dropped: 0,
        }
    }

    pub fn increment_sent(&mut self, bytes: usize) {
        self.bytes_sent += bytes as u64;
        self.packets_sent += 1;
    }

    pub fn increment_received(&mut self, bytes: usize) {
        self.bytes_received += bytes as u64;
        self.packets_received += 1;
    }

    pub fn increment_errors(&mut self) {
        self.errors += 1;
    }

    pub fn increment_dropped(&mut self) {
        self.dropped += 1;
    }
}

impl Default for SocketStats {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SocketInfo {
    pub id: SocketId,
    pub socket_type: SocketType,
    pub protocol: Protocol,
    pub local_addr: [u8; 4],
    pub local_port: u16,
    pub remote_addr: Option<[u8; 4]>,
    pub remote_port: u16,
    pub state: Option<SocketState>,
    pub stats: SocketStats,
}

impl SocketInfo {
    pub fn format(&self) -> alloc::string::String {
        let type_str = match self.socket_type {
            SocketType::Stream => "STREAM",
            SocketType::Datagram => "DGRAM",
            SocketType::Raw => "RAW",
        };

        let protocol_str = self.protocol.as_str();

        let na = alloc::string::String::from("N/A");
        let state_str = self
            .state
            .map(|s| alloc::format!("{:?}", s))
            .unwrap_or_else(|| na.clone());

        let local = alloc::format!(
            "{}.{}.{}.{}:{}",
            self.local_addr[0],
            self.local_addr[1],
            self.local_addr[2],
            self.local_addr[3],
            self.local_port
        );

        let remote = self
            .remote_addr
            .map(|addr| {
                alloc::format!(
                    "{}.{}.{}.{}:{}",
                    addr[0],
                    addr[1],
                    addr[2],
                    addr[3],
                    self.remote_port
                )
            })
            .unwrap_or_else(|| na.clone());

        alloc::format!(
            "Socket {:>3} | {:>6} | {:>3} | {:>21} | {:>21} | {:>15}",
            self.id.0,
            type_str,
            protocol_str,
            local,
            remote,
            state_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_id() {
        let id = SocketId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_socket_stats() {
        let mut stats = SocketStats::new();

        stats.increment_sent(100);
        stats.increment_received(50);

        assert_eq!(stats.bytes_sent, 100);
        assert_eq!(stats.bytes_received, 50);
        assert_eq!(stats.packets_sent, 1);
        assert_eq!(stats.packets_received, 1);
    }
}
