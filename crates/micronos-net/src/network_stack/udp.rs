use alloc::vec::Vec;
use spin::RwLock;

use crate::network_stack::protocol::{IpEndpoint, Port, UdpHeader};
use crate::network_stack::socket::{SocketId, UdpSocket};

#[derive(Debug, Clone)]
pub struct UdpDatagram {
    pub header: UdpHeader,
    pub payload: Vec<u8>,
}

impl UdpDatagram {
    pub fn new(src_port: u16, dst_port: u16, payload: Vec<u8>) -> Self {
        let length = 8 + payload.len() as u16;

        UdpDatagram {
            header: UdpHeader {
                src_port,
                dst_port,
                length,
                checksum: 0,
            },
            payload,
        }
    }

    pub fn from_endpoints(local: IpEndpoint, remote: IpEndpoint, payload: Vec<u8>) -> Self {
        Self::new(local.port.value(), remote.port.value(), payload)
    }

    pub fn calculate_checksum(&self, src_addr: &[u8; 4], dst_addr: &[u8; 4]) -> u16 {
        let mut sum: u32 = 0;

        sum += (src_addr[0] as u32) << 8 | src_addr[1] as u32;
        sum += (src_addr[2] as u32) << 8 | src_addr[3] as u32;
        sum += (dst_addr[0] as u32) << 8 | dst_addr[1] as u32;
        sum += (dst_addr[2] as u32) << 8 | dst_addr[3] as u32;
        sum += 17u32;
        sum += self.header.length as u32;

        sum += self.header.src_port as u32;
        sum += self.header.dst_port as u32;
        sum += self.header.length as u32;

        for &byte in &self.payload {
            sum += byte as u32;
        }

        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }

    pub fn is_valid(&self) -> bool {
        self.header.length >= 8 && self.header.length as usize <= self.payload.len() + 8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UdpError {
    PortUnreachable,
    BufferFull,
    InvalidPacket,
    ChecksumMismatch,
    NotBound,
}

impl core::fmt::Display for UdpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UdpError::PortUnreachable => write!(f, "Port unreachable"),
            UdpError::BufferFull => write!(f, "Buffer full"),
            UdpError::InvalidPacket => write!(f, "Invalid UDP packet"),
            UdpError::ChecksumMismatch => write!(f, "Checksum mismatch"),
            UdpError::NotBound => write!(f, "Socket not bound"),
        }
    }
}

pub struct UdpStack {
    sockets: RwLock<Vec<UdpSocket>>,
    next_socket_id: RwLock<u32>,
    receive_buffer: RwLock<Vec<UdpDatagram>>,
}

impl UdpStack {
    pub fn new() -> Self {
        UdpStack {
            sockets: RwLock::new(Vec::new()),
            next_socket_id: RwLock::new(1),
            receive_buffer: RwLock::new(Vec::new()),
        }
    }

    pub fn create_socket(&self, local: IpEndpoint) -> Result<SocketId, UdpError> {
        let mut id_counter = self.next_socket_id.write();
        let id = SocketId(*id_counter);
        *id_counter += 1;

        let socket = UdpSocket::new(id, local);

        self.sockets.write().push(socket);

        Ok(id)
    }

    pub fn get_socket(&self, id: SocketId) -> Option<UdpSocket> {
        self.sockets.read().iter().find(|s| s.id == id).cloned()
    }

    pub fn remove_socket(&self, id: SocketId) -> Option<UdpSocket> {
        let mut sockets = self.sockets.write();
        sockets
            .iter()
            .position(|s| s.id == id)
            .map(|pos| sockets.remove(pos))
    }

    pub fn send_to(
        &self,
        id: SocketId,
        remote: IpEndpoint,
        data: Vec<u8>,
    ) -> Result<usize, UdpError> {
        let local_addr_copy;
        let local_port_copy;
        let data_len = data.len();

        {
            let socket = self
                .sockets
                .read()
                .iter()
                .find(|s| s.id == id)
                .cloned()
                .ok_or(UdpError::NotBound)?;

            local_addr_copy = socket.local_addr;
            local_port_copy = socket.local_port;
        }

        let _datagram = UdpDatagram::from_endpoints(
            IpEndpoint::new(
                core::net::Ipv4Addr::new(
                    local_addr_copy[0],
                    local_addr_copy[1],
                    local_addr_copy[2],
                    local_addr_copy[3],
                ),
                Port::new(local_port_copy),
            ),
            remote,
            data,
        );

        Ok(data_len)
    }

    pub fn receive_from(&self, id: SocketId) -> Result<Option<(IpEndpoint, Vec<u8>)>, UdpError> {
        let socket = self
            .sockets
            .read()
            .iter()
            .find(|s| s.id == id)
            .cloned()
            .ok_or(UdpError::NotBound)?;

        let buffer = self.receive_buffer.read();

        if let Some(datagram) = buffer
            .iter()
            .find(|d| d.header.dst_port == socket.local_port)
        {
            let remote = IpEndpoint::any(Port::new(datagram.header.src_port));
            Ok(Some((remote, datagram.payload.clone())))
        } else {
            Ok(None)
        }
    }

    pub fn queue_datagram(&self, datagram: UdpDatagram) {
        self.receive_buffer.write().push(datagram);
    }

    pub fn socket_count(&self) -> usize {
        self.sockets.read().len()
    }

    pub fn queued_packets(&self) -> usize {
        self.receive_buffer.read().len()
    }
}

impl Default for UdpStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use core::net::Ipv4Addr;

    #[test]
    fn test_udp_datagram_creation() {
        let payload = vec![1, 2, 3, 4, 5];
        let datagram = UdpDatagram::new(8080, 80, payload.clone());

        assert_eq!(datagram.header.src_port, 8080);
        assert_eq!(datagram.header.dst_port, 80);
        assert_eq!(datagram.header.length, 8 + 5);
        assert_eq!(datagram.payload, payload);
    }

    #[test]
    fn test_udp_datagram_validity() {
        let valid = UdpDatagram::new(8080, 80, vec![1, 2, 3]);
        assert!(valid.is_valid());
    }

    #[test]
    fn test_udp_stack_socket_creation() {
        let stack = UdpStack::new();
        let endpoint = IpEndpoint::local(Port::new(8080));

        let id = stack.create_socket(endpoint).unwrap();

        assert_eq!(id.0, 1);
        assert_eq!(stack.socket_count(), 1);
    }

    #[test]
    fn test_udp_stack_send_receive() {
        let stack = UdpStack::new();
        let local = IpEndpoint::local(Port::new(8080));

        let id = stack.create_socket(local).unwrap();

        let remote = IpEndpoint::new(Ipv4Addr::new(192, 168, 1, 1), Port::new(80));
        let data = vec![1, 2, 3, 4, 5];

        let sent = stack.send_to(id, remote, data.clone()).unwrap();
        assert_eq!(sent, 5);
    }
}
