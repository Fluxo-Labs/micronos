use alloc::string::String;
use alloc::vec::Vec;
use micronos_core::{error::Error, types::NodeId};

#[derive(Debug, Clone, Default)]
pub enum P2PState {
    #[default]
    Disconnected,
    Discovering,
    Handshaking,
    Connected,
    Disconnecting,
}

pub struct P2PStack {
    pub state: P2PState,
    pub local_id: NodeId,
    pub peers: Vec<PeerInfo>,
}

impl P2PStack {
    pub fn new(local_id: NodeId) -> Self {
        P2PStack {
            state: P2PState::Disconnected,
            local_id,
            peers: Vec::new(),
        }
    }

    pub fn local_node_id(&self) -> NodeId {
        self.local_id
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn start_discovery(&mut self) -> Result<(), Error> {
        if matches!(self.state, P2PState::Disconnected) {
            self.state = P2PState::Discovering;
            Ok(())
        } else {
            Err(Error::InvalidState)
        }
    }

    pub fn initiate_handshake(&mut self, _peer: NodeId) -> Result<(), Error> {
        if matches!(self.state, P2PState::Discovering) {
            self.state = P2PState::Handshaking;
            Ok(())
        } else {
            Err(Error::InvalidState)
        }
    }

    pub fn complete_connection(&mut self) -> Result<(), Error> {
        if matches!(self.state, P2PState::Handshaking) {
            self.state = P2PState::Connected;
            Ok(())
        } else {
            Err(Error::InvalidState)
        }
    }

    pub fn begin_disconnect(&mut self) -> Result<(), Error> {
        if matches!(self.state, P2PState::Connected) {
            self.state = P2PState::Disconnecting;
            Ok(())
        } else {
            Err(Error::InvalidState)
        }
    }

    pub fn finish_disconnect(&mut self) -> Result<(), Error> {
        if matches!(self.state, P2PState::Disconnecting) {
            self.state = P2PState::Disconnected;
            self.peers.clear();
            Ok(())
        } else {
            Err(Error::InvalidState)
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.state, P2PState::Connected)
    }
}

impl Default for P2PStack {
    fn default() -> Self {
        Self::new(NodeId::default())
    }
}

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub id: NodeId,
    pub addr: String,
    pub latency_ms: u32,
    pub connected: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2p_creation() {
        let p2p = P2PStack::new(NodeId::default());
        assert!(matches!(p2p.state, P2PState::Disconnected));
    }

    #[test]
    fn test_discovery_workflow() {
        let mut p2p = P2PStack::new(NodeId::default());

        p2p.start_discovery().unwrap();
        assert!(matches!(p2p.state, P2PState::Discovering));

        p2p.initiate_handshake(NodeId::default()).unwrap();
        assert!(matches!(p2p.state, P2PState::Handshaking));

        p2p.complete_connection().unwrap();
        assert!(matches!(p2p.state, P2PState::Connected));

        p2p.begin_disconnect().unwrap();
        assert!(matches!(p2p.state, P2PState::Disconnecting));

        p2p.finish_disconnect().unwrap();
        assert!(matches!(p2p.state, P2PState::Disconnected));
    }

    #[test]
    fn test_peer_count() {
        let p2p = P2PStack::new(NodeId::default());
        assert_eq!(p2p.peer_count(), 0);
    }
}
