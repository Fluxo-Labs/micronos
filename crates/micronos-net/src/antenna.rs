use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use micronos_core::{error::Error, types::NodeId};

#[derive(Debug, Clone, Default)]
pub enum AntennaState {
    #[default]
    Offline,
    Discovering,
    Connected {
        peer: NodeId,
        signal_strength: f32,
    },
    Disconnected,
    Faulted,
}

pub struct MicronetAntenna {
    pub state: AntennaState,
    #[allow(dead_code)]
    pub freq_mhz: u32,
    #[allow(dead_code)]
    pub power_dbm: i32,
}

impl MicronetAntenna {
    pub const MAX_FREQ_MHZ: u32 = 6000;
    pub const MIN_FREQ_MHZ: u32 = 2400;

    pub fn new() -> Self {
        MicronetAntenna {
            state: AntennaState::Offline,
            freq_mhz: 2400,
            power_dbm: 0,
        }
    }

    pub fn with_config(freq_mhz: u32, power_dbm: i32) -> Result<Self, Error> {
        if !(Self::MIN_FREQ_MHZ..=Self::MAX_FREQ_MHZ).contains(&freq_mhz) {
            return Err(Error::network("Invalid frequency"));
        }
        Ok(MicronetAntenna {
            state: AntennaState::Offline,
            freq_mhz,
            power_dbm,
        })
    }

    pub fn discover(&mut self) {
        if matches!(self.state, AntennaState::Offline) {
            self.state = AntennaState::Discovering;
        }
    }

    pub fn connect_to(&mut self, node: NodeId) -> Result<(), Error> {
        if matches!(self.state, AntennaState::Discovering) {
            self.state = AntennaState::Connected {
                peer: node,
                signal_strength: -50.0,
            };
            Ok(())
        } else {
            Err(Error::InvalidState)
        }
    }

    pub fn disconnect(&mut self) {
        if matches!(self.state, AntennaState::Connected { .. }) {
            self.state = AntennaState::Disconnected;
        }
    }

    pub fn recover(&mut self) {
        self.state = AntennaState::Offline;
    }

    pub fn scan_channels(&self) -> Vec<ChannelInfo> {
        vec![
            ChannelInfo {
                freq: 2412,
                ssid: Some("Network1".to_string()),
                rssi: -45.0,
            },
            ChannelInfo {
                freq: 2437,
                ssid: Some("Network2".to_string()),
                rssi: -60.0,
            },
            ChannelInfo {
                freq: 5180,
                ssid: Some("Network5G".to_string()),
                rssi: -55.0,
            },
        ]
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.state, AntennaState::Connected { .. })
    }
}

impl Default for MicronetAntenna {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub freq: u32,
    pub ssid: Option<String>,
    pub rssi: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antenna_creation() {
        let antenna = MicronetAntenna::new();
        assert!(matches!(antenna.state, AntennaState::Offline));
    }

    #[test]
    fn test_antenna_config() {
        let antenna = MicronetAntenna::with_config(5200, 20);
        assert!(antenna.is_ok());
    }

    #[test]
    fn test_antenna_invalid_freq() {
        let antenna = MicronetAntenna::with_config(100, 20);
        assert!(antenna.is_err());
    }

    #[test]
    fn test_channel_scan() {
        let antenna = MicronetAntenna::new();
        let channels = antenna.scan_channels();
        assert!(!channels.is_empty());
    }

    #[test]
    fn test_discover_workflow() {
        let mut antenna = MicronetAntenna::new();
        antenna.discover();
        assert!(matches!(antenna.state, AntennaState::Discovering));

        let node_id = NodeId::default();
        antenna.connect_to(node_id).unwrap();
        assert!(matches!(antenna.state, AntennaState::Connected { .. }));

        antenna.disconnect();
        assert!(matches!(antenna.state, AntennaState::Disconnected));
    }
}
