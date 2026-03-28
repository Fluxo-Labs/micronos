use core::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
}

impl Protocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Protocol::Tcp => "TCP",
            Protocol::Udp => "UDP",
            Protocol::Icmp => "ICMP",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Port(u16);

impl Port {
    pub fn new(port: u16) -> Self {
        Port(port)
    }

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn is_privileged(&self) -> bool {
        self.0 < 1024
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpEndpoint {
    pub addr: Ipv4Addr,
    pub port: Port,
}

impl IpEndpoint {
    pub fn new(addr: Ipv4Addr, port: Port) -> Self {
        IpEndpoint { addr, port }
    }

    pub fn local(port: Port) -> Self {
        IpEndpoint {
            addr: Ipv4Addr::LOCALHOST,
            port,
        }
    }

    pub fn any(port: Port) -> Self {
        IpEndpoint {
            addr: Ipv4Addr::UNSPECIFIED,
            port,
        }
    }

    pub fn to_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(self.addr), self.port.value())
    }
}

impl Default for IpEndpoint {
    fn default() -> Self {
        Self::any(Port::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SocketState {
    #[default]
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    Closing,
    TimeWait,
    CloseWait,
    LastAck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream,
    Datagram,
    Raw,
}

impl SocketType {
    pub fn protocol(&self) -> Protocol {
        match self {
            SocketType::Stream => Protocol::Tcp,
            SocketType::Datagram => Protocol::Udp,
            SocketType::Raw => Protocol::Icmp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: TcpFlags,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TcpFlags {
    pub fin: bool,
    pub syn: bool,
    pub rst: bool,
    pub psh: bool,
    pub ack: bool,
    pub urg: bool,
}

impl TcpFlags {
    pub fn new() -> Self {
        TcpFlags::default()
    }

    pub fn syn() -> Self {
        TcpFlags {
            syn: true,
            ..Default::default()
        }
    }

    pub fn ack() -> Self {
        TcpFlags {
            ack: true,
            ..Default::default()
        }
    }

    pub fn fin() -> Self {
        TcpFlags {
            fin: true,
            ..Default::default()
        }
    }

    pub fn fin_ack() -> Self {
        TcpFlags {
            fin: true,
            ack: true,
            ..Default::default()
        }
    }

    pub fn rst() -> Self {
        TcpFlags {
            rst: true,
            ..Default::default()
        }
    }

    pub fn syn_ack() -> Self {
        TcpFlags {
            syn: true,
            ack: true,
            ..Default::default()
        }
    }
}

impl Default for TcpHeader {
    fn default() -> Self {
        TcpHeader {
            src_port: 0,
            dst_port: 0,
            seq: 0,
            ack: 0,
            flags: TcpFlags::default(),
            window_size: 65535,
            checksum: 0,
            urgent_ptr: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl Default for UdpHeader {
    fn default() -> Self {
        UdpHeader {
            src_port: 0,
            dst_port: 0,
            length: 8,
            checksum: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpHeader {
    pub version: U4,
    pub ihl: U4,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: IpFlags,
    pub fragment_offset: U13,
    pub ttl: u8,
    pub protocol: Protocol,
    pub checksum: u16,
    pub src_addr: Ipv4Addr,
    pub dst_addr: Ipv4Addr,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IpFlags {
    pub reserved: bool,
    pub dont_fragment: bool,
    pub more_fragments: bool,
}

#[derive(Debug, Clone, Copy, Default)]
#[allow(dead_code)]
pub struct U4(u8);

#[derive(Debug, Clone, Copy, Default)]
#[allow(dead_code)]
pub struct U13(u16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port() {
        let port = Port::new(8080);
        assert_eq!(port.value(), 8080);
        assert!(!port.is_privileged());

        let privileged = Port::new(80);
        assert!(privileged.is_privileged());
    }

    #[test]
    fn test_ip_endpoint() {
        let endpoint = IpEndpoint::local(Port::new(8080));
        assert_eq!(endpoint.addr, Ipv4Addr::LOCALHOST);
        assert_eq!(endpoint.port.value(), 8080);
    }

    #[test]
    fn test_tcp_flags() {
        let syn = TcpFlags::syn();
        assert!(syn.syn);
        assert!(!syn.ack);
        assert!(!syn.fin);

        let fin_ack = TcpFlags::fin_ack();
        assert!(fin_ack.fin);
        assert!(fin_ack.ack);
    }

    #[test]
    fn test_socket_state() {
        let state = SocketState::Closed;
        assert_eq!(state, SocketState::Closed);
    }
}
