use micronos_core::types::NodeId;
use micronos_net::antenna::AntennaState;
use micronos_net::p2p::P2PState;
use micronos_net::{MicronetAntenna, P2PStack};

#[test]
fn test_antenna_channel_scan() {
    let antenna = MicronetAntenna::new();
    let channels = antenna.scan_channels();
    assert!(!channels.is_empty());
}

#[test]
fn test_antenna_workflow() {
    let mut antenna = MicronetAntenna::new();
    antenna.discover();
    assert!(matches!(antenna.state, AntennaState::Discovering));

    antenna.connect_to(NodeId::default()).unwrap();
    assert!(matches!(antenna.state, AntennaState::Connected { .. }));

    antenna.disconnect();
    assert!(matches!(antenna.state, AntennaState::Disconnected));
}

#[test]
fn test_p2p_workflow() {
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
fn test_p2p_local_id() {
    let node_id = NodeId::default();
    let p2p = P2PStack::new(node_id);
    assert_eq!(p2p.local_node_id(), node_id);
}
