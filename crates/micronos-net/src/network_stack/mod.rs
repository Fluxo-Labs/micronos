pub mod protocol;
pub mod socket;
pub mod tcp;
pub mod udp;

pub use protocol::{IpEndpoint, Port, Protocol, SocketState, SocketType};
pub use socket::{SocketError, SocketId, SocketInfo, SocketStats, TcpSocket, UdpSocket};
pub use tcp::{TcpConnection, TcpError, TcpPacket, TcpStack};
pub use udp::{UdpDatagram, UdpError, UdpStack};

pub struct NetworkStack {
    pub tcp: TcpStack,
    pub udp: UdpStack,
}

impl NetworkStack {
    pub fn new() -> Self {
        NetworkStack {
            tcp: TcpStack::new(),
            udp: UdpStack::new(),
        }
    }

    pub fn total_sockets(&self) -> usize {
        self.tcp.connection_count() + self.udp.socket_count()
    }

    pub fn active_connections(&self) -> usize {
        self.tcp.active_connections()
    }

    pub fn format_status(&self) -> alloc::string::String {
        let mut output = alloc::string::String::from(
            "╔════════════════════════════════════════════════════════════════╗\n",
        );
        output.push_str("║                    Network Stack                          ║\n");
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");
        output.push_str(&alloc::format!(
            "║  TCP Connections: {:>3} (active: {:>3})                          ║\n",
            self.tcp.connection_count(),
            self.tcp.active_connections()
        ));
        output.push_str(&alloc::format!(
            "║  UDP Sockets:     {:>3}                                        ║\n",
            self.udp.socket_count()
        ));
        output.push_str(&alloc::format!(
            "║  Queued Packets:  {:>3}                                        ║\n",
            self.udp.queued_packets()
        ));
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");
        output.push_str("║  ID    │ Type   │ Proto │ Local Address        │ Remote Address       │ State             ║\n");
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");

        let tcp_count = self.tcp.connection_count();
        let udp_count = self.udp.socket_count();

        if tcp_count == 0 && udp_count == 0 {
            output.push_str("║  No sockets active                                            ║\n");
        } else {
            for i in 0..tcp_count {
                let conn = self.tcp.get_connection(SocketId(i as u32 + 1));
                if let Some(c) = conn {
                    let state_str = alloc::format!("{:?}", c.state);
                    let local_addr = alloc::format!(
                        "{}.{}.{}.{}",
                        c.local_addr[0],
                        c.local_addr[1],
                        c.local_addr[2],
                        c.local_addr[3]
                    );
                    let remote_addr = alloc::format!(
                        "{}.{}.{}.{}",
                        c.remote_addr[0],
                        c.remote_addr[1],
                        c.remote_addr[2],
                        c.remote_addr[3]
                    );
                    output.push_str(&alloc::format!(
                        "║  {:>5} │ STREAM │ TCP   │ {:>20}:{:>5} │ {:>20}:{:>5} │ {:>16} ║\n",
                        c.socket_id.0,
                        local_addr,
                        c.local_port,
                        remote_addr,
                        c.remote_port,
                        state_str
                    ));
                }
            }

            for i in 0..udp_count {
                let socket = self.udp.get_socket(SocketId(i as u32 + 1));
                if let Some(s) = socket {
                    let na = alloc::string::String::from("N/A");
                    let local_addr = alloc::format!(
                        "{}.{}.{}.{}",
                        s.local_addr[0],
                        s.local_addr[1],
                        s.local_addr[2],
                        s.local_addr[3]
                    );
                    let remote = s
                        .remote_addr
                        .map(|addr| {
                            alloc::format!(
                                "{}.{}.{}.{}:{}",
                                addr[0],
                                addr[1],
                                addr[2],
                                addr[3],
                                s.remote_port
                            )
                        })
                        .unwrap_or_else(|| na.clone());
                    output.push_str(&alloc::format!(
                        "║  {:>5} │ DGRAM  │ UDP   │ {:>20}:{:>5} │ {:>21} │ {:>16} ║\n",
                        s.id.0,
                        local_addr,
                        s.local_port,
                        remote,
                        "N/A"
                    ));
                }
            }
        }

        output.push_str("╚════════════════════════════════════════════════════════════════╝");
        output
    }
}

impl Default for NetworkStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_stack_creation() {
        let stack = NetworkStack::new();
        assert_eq!(stack.total_sockets(), 0);
        assert_eq!(stack.active_connections(), 0);
    }

    #[test]
    fn test_tcp_socket_creation() {
        let stack = NetworkStack::new();

        let local = IpEndpoint::local(Port::new(8080));
        let remote = IpEndpoint::new(core::net::Ipv4Addr::new(192, 168, 1, 1), Port::new(80));

        let id = stack.tcp.create_connection(local, remote).unwrap();
        assert_eq!(id.0, 1);
        assert_eq!(stack.total_sockets(), 1);
    }

    #[test]
    fn test_udp_socket_creation() {
        let stack = NetworkStack::new();

        let local = IpEndpoint::local(Port::new(8080));

        let id = stack.udp.create_socket(local).unwrap();
        assert_eq!(id.0, 1);
        assert_eq!(stack.total_sockets(), 1);
    }
}
