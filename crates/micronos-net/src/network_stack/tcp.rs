use alloc::vec::Vec;
use spin::RwLock;

use crate::network_stack::protocol::{IpEndpoint, SocketState, TcpFlags, TcpHeader};
use crate::network_stack::socket::SocketId;

#[derive(Debug, Clone)]
pub struct TcpConnection {
    pub socket_id: SocketId,
    pub local_addr: [u8; 4],
    pub local_port: u16,
    pub remote_addr: [u8; 4],
    pub remote_port: u16,
    pub state: SocketState,
    seq: u32,
    ack: u32,
    window_size: u16,
    retransmit_queue: Vec<Vec<u8>>,
    pub local_seq: u32,
    pub remote_seq: u32,
}

impl TcpConnection {
    pub fn new(socket_id: SocketId, local: IpEndpoint, remote: IpEndpoint) -> Self {
        let local_bytes = local.addr.octets();
        let remote_bytes = remote.addr.octets();

        TcpConnection {
            socket_id,
            local_addr: local_bytes,
            local_port: local.port.value(),
            remote_addr: remote_bytes,
            remote_port: remote.port.value(),
            state: SocketState::Closed,
            seq: 1000,
            ack: 0,
            window_size: 65535,
            retransmit_queue: Vec::new(),
            local_seq: 1000,
            remote_seq: 0,
        }
    }

    pub fn connect(&mut self) -> Result<(), TcpError> {
        if self.state != SocketState::Closed {
            return Err(TcpError::InvalidState);
        }

        self.state = SocketState::SynSent;
        self.local_seq = self.seq;

        Ok(())
    }

    pub fn listen(&mut self) -> Result<(), TcpError> {
        if self.state != SocketState::Closed {
            return Err(TcpError::InvalidState);
        }

        self.state = SocketState::Listen;
        Ok(())
    }

    pub fn send_syn(&mut self) -> TcpPacket {
        let header = TcpHeader {
            src_port: self.local_port,
            dst_port: self.remote_port,
            seq: self.local_seq,
            ack: 0,
            flags: TcpFlags::syn(),
            window_size: self.window_size,
            checksum: 0,
            urgent_ptr: 0,
        };

        self.state = SocketState::SynSent;

        TcpPacket {
            header,
            payload: Vec::new(),
        }
    }

    pub fn receive_syn(&mut self, packet: &TcpPacket) -> Result<TcpPacket, TcpError> {
        if packet.header.flags.syn {
            self.remote_seq = packet.header.seq;
            self.ack = packet.header.seq + 1;
            self.state = SocketState::SynReceived;

            let response = TcpHeader {
                src_port: self.local_port,
                dst_port: self.remote_port,
                seq: self.local_seq,
                ack: self.ack,
                flags: TcpFlags::syn_ack(),
                window_size: self.window_size,
                checksum: 0,
                urgent_ptr: 0,
            };

            Ok(TcpPacket {
                header: response,
                payload: Vec::new(),
            })
        } else {
            Err(TcpError::UnexpectedPacket)
        }
    }

    pub fn receive_syn_ack(&mut self, packet: &TcpPacket) -> Result<TcpPacket, TcpError> {
        if packet.header.flags.syn && packet.header.flags.ack {
            self.remote_seq = packet.header.seq;
            self.ack = packet.header.seq + 1;
            self.local_seq += 1;
            self.state = SocketState::Established;

            let response = TcpHeader {
                src_port: self.local_port,
                dst_port: self.remote_port,
                seq: self.local_seq,
                ack: self.ack,
                flags: TcpFlags::ack(),
                window_size: self.window_size,
                checksum: 0,
                urgent_ptr: 0,
            };

            Ok(TcpPacket {
                header: response,
                payload: Vec::new(),
            })
        } else {
            Err(TcpError::UnexpectedPacket)
        }
    }

    pub fn send_data(&mut self, data: Vec<u8>) -> Result<TcpPacket, TcpError> {
        if self.state != SocketState::Established {
            return Err(TcpError::InvalidState);
        }

        let data_len = data.len();

        let packet = TcpPacket {
            header: TcpHeader {
                src_port: self.local_port,
                dst_port: self.remote_port,
                seq: self.local_seq,
                ack: self.ack,
                flags: TcpFlags {
                    psh: true,
                    ack: true,
                    ..TcpFlags::default()
                },
                window_size: self.window_size,
                checksum: 0,
                urgent_ptr: 0,
            },
            payload: data.clone(),
        };

        self.local_seq += data_len as u32;
        self.retransmit_queue.push(data);

        Ok(packet)
    }

    pub fn receive_data(&mut self, packet: &TcpPacket) -> Result<Vec<u8>, TcpError> {
        if self.state != SocketState::Established {
            return Err(TcpError::InvalidState);
        }

        if packet.header.flags.ack {
            self.ack = packet.header.seq + packet.payload.len() as u32;

            if !self.retransmit_queue.is_empty() {
                self.retransmit_queue.remove(0);
            }
        }

        if !packet.payload.is_empty() {
            self.ack = packet.header.seq + packet.payload.len() as u32;
            return Ok(packet.payload.clone());
        }

        Ok(Vec::new())
    }

    pub fn close(&mut self) -> Result<TcpPacket, TcpError> {
        match self.state {
            SocketState::Established | SocketState::CloseWait => {
                self.state = SocketState::FinWait1;

                let packet = TcpPacket {
                    header: TcpHeader {
                        src_port: self.local_port,
                        dst_port: self.remote_port,
                        seq: self.local_seq,
                        ack: self.ack,
                        flags: TcpFlags::fin(),
                        window_size: self.window_size,
                        checksum: 0,
                        urgent_ptr: 0,
                    },
                    payload: Vec::new(),
                };

                self.local_seq += 1;
                Ok(packet)
            }
            _ => Err(TcpError::InvalidState),
        }
    }

    pub fn receive_fin(&mut self, packet: &TcpPacket) -> Result<TcpPacket, TcpError> {
        if packet.header.flags.fin {
            self.ack = packet.header.seq + 1;

            let new_state = match self.state {
                SocketState::FinWait1 => SocketState::Closing,
                SocketState::FinWait2 => SocketState::TimeWait,
                _ => self.state,
            };

            self.state = new_state;

            let response = TcpPacket {
                header: TcpHeader {
                    src_port: self.local_port,
                    dst_port: self.remote_port,
                    seq: self.local_seq,
                    ack: self.ack,
                    flags: TcpFlags::ack(),
                    window_size: self.window_size,
                    checksum: 0,
                    urgent_ptr: 0,
                },
                payload: Vec::new(),
            };

            Ok(response)
        } else {
            Err(TcpError::UnexpectedPacket)
        }
    }

    pub fn state(&self) -> SocketState {
        self.state
    }

    pub fn is_connected(&self) -> bool {
        self.state == SocketState::Established
    }
}

#[derive(Debug, Clone)]
pub struct TcpPacket {
    pub header: TcpHeader,
    pub payload: Vec<u8>,
}

impl TcpPacket {
    pub fn new(header: TcpHeader, payload: Vec<u8>) -> Self {
        TcpPacket { header, payload }
    }

    pub fn calculate_checksum(&self) -> u16 {
        let mut sum: u32 = 0;

        sum += self.header.src_port as u32;
        sum += self.header.dst_port as u32;
        sum += self.header.seq >> 16;
        sum += self.header.seq & 0xFFFF;
        sum += self.header.ack >> 16;
        sum += self.header.ack & 0xFFFF;

        for &byte in &self.payload {
            if sum & 1 == 1 {
                sum = (sum >> 1) + 0x8000;
            } else {
                sum >>= 1;
            }
            sum += byte as u32;
        }

        !sum as u16
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpError {
    InvalidState,
    ConnectionRefused,
    ConnectionReset,
    ConnectionTimeout,
    UnexpectedPacket,
    ChecksumMismatch,
    BufferFull,
}

impl core::fmt::Display for TcpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TcpError::InvalidState => write!(f, "Invalid TCP state"),
            TcpError::ConnectionRefused => write!(f, "Connection refused"),
            TcpError::ConnectionReset => write!(f, "Connection reset"),
            TcpError::ConnectionTimeout => write!(f, "Connection timeout"),
            TcpError::UnexpectedPacket => write!(f, "Unexpected packet"),
            TcpError::ChecksumMismatch => write!(f, "Checksum mismatch"),
            TcpError::BufferFull => write!(f, "Buffer full"),
        }
    }
}

pub struct TcpStack {
    connections: RwLock<Vec<TcpConnection>>,
    next_connection_id: RwLock<u32>,
}

impl TcpStack {
    pub fn new() -> Self {
        TcpStack {
            connections: RwLock::new(Vec::new()),
            next_connection_id: RwLock::new(1),
        }
    }

    pub fn create_connection(
        &self,
        local: IpEndpoint,
        remote: IpEndpoint,
    ) -> Result<SocketId, TcpError> {
        let mut id_counter = self.next_connection_id.write();
        let id = SocketId(*id_counter);
        *id_counter += 1;

        let connection = TcpConnection::new(id, local, remote);

        self.connections.write().push(connection);

        Ok(id)
    }

    pub fn get_connection(&self, id: SocketId) -> Option<TcpConnection> {
        self.connections
            .read()
            .iter()
            .find(|c| c.socket_id == id)
            .cloned()
    }

    pub fn remove_connection(&self, id: SocketId) -> Option<TcpConnection> {
        let mut connections = self.connections.write();
        connections
            .iter()
            .position(|c| c.socket_id == id)
            .map(|pos| connections.remove(pos))
    }

    pub fn connection_count(&self) -> usize {
        self.connections.read().len()
    }

    pub fn active_connections(&self) -> usize {
        self.connections
            .read()
            .iter()
            .filter(|c| c.is_connected())
            .count()
    }
}

impl Default for TcpStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network_stack::protocol::Port;
    use core::net::Ipv4Addr;

    #[test]
    fn test_tcp_connection_creation() {
        let local = IpEndpoint::local(Port::new(8080));
        let remote = IpEndpoint::new(Ipv4Addr::new(192, 168, 1, 1), Port::new(80));

        let conn = TcpConnection::new(SocketId(1), local, remote);

        assert_eq!(conn.state(), SocketState::Closed);
        assert!(!conn.is_connected());
    }

    #[test]
    fn test_tcp_connect() {
        let local = IpEndpoint::local(Port::new(8080));
        let remote = IpEndpoint::new(Ipv4Addr::new(192, 168, 1, 1), Port::new(80));

        let mut conn = TcpConnection::new(SocketId(1), local, remote);
        conn.connect().unwrap();

        assert_eq!(conn.state(), SocketState::SynSent);
    }

    #[test]
    fn test_tcp_listen() {
        let local = IpEndpoint::any(Port::new(8080));

        let mut conn = TcpConnection::new(SocketId(1), local, IpEndpoint::default());
        conn.listen().unwrap();

        assert_eq!(conn.state(), SocketState::Listen);
    }

    #[test]
    fn test_tcp_syn_packet() {
        let mut conn = TcpConnection::new(
            SocketId(1),
            IpEndpoint::local(Port::new(8080)),
            IpEndpoint::new(Ipv4Addr::new(192, 168, 1, 1), Port::new(80)),
        );

        let syn_packet = conn.send_syn();

        assert!(syn_packet.header.flags.syn);
        assert!(!syn_packet.header.flags.ack);
    }
}
