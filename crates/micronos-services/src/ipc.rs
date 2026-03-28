use alloc::string::{String, ToString};
use alloc::vec::Vec;
use micronos_core::error::Error;
use micronos_core::types::ProcessId;

#[derive(Debug, Clone, Default)]
pub enum ChannelState {
    #[default]
    Open,
    Closed,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub from: ProcessId,
    pub to: ProcessId,
    pub payload: Vec<u8>,
    pub priority: u8,
}

impl Message {
    pub fn new(from: ProcessId, to: ProcessId, payload: Vec<u8>) -> Self {
        Message {
            from,
            to,
            payload,
            priority: 0,
        }
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn payload_str(&self) -> Option<String> {
        String::from_utf8(self.payload.clone()).ok()
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub sender: ProcessId,
    pub receiver: ProcessId,
    pub messages: Vec<Message>,
    pub state: ChannelState,
}

impl Channel {
    pub fn new(id: ChannelId, name: &str, sender: ProcessId, receiver: ProcessId) -> Self {
        Channel {
            id,
            name: name.to_string(),
            sender,
            receiver,
            messages: Vec::new(),
            state: ChannelState::Open,
        }
    }

    pub fn send(&mut self, message: Message) {
        if matches!(self.state, ChannelState::Open) {
            self.messages.push(message);
        }
    }

    pub fn receive(&mut self) -> Option<Message> {
        if matches!(self.state, ChannelState::Open) && !self.messages.is_empty() {
            Some(self.messages.remove(0))
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&Message> {
        self.messages.first()
    }

    pub fn close(&mut self) {
        self.state = ChannelState::Closed;
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChannelId(pub u64);

pub struct IpcManager {
    channels: Vec<Channel>,
    next_channel_id: u64,
}

impl IpcManager {
    pub fn new() -> Self {
        IpcManager {
            channels: Vec::new(),
            next_channel_id: 1,
        }
    }

    pub fn create_channel(
        &mut self,
        name: &str,
        sender: ProcessId,
        receiver: ProcessId,
    ) -> Result<ChannelId, Error> {
        let id = ChannelId(self.next_channel_id);
        self.next_channel_id += 1;

        let channel = Channel::new(id, name, sender, receiver);
        self.channels.push(channel);
        Ok(id)
    }

    pub fn find_channel(&self, id: ChannelId) -> Option<&Channel> {
        self.channels.iter().find(|c| c.id == id)
    }

    pub fn find_channel_mut(&mut self, id: ChannelId) -> Option<&mut Channel> {
        self.channels.iter_mut().find(|c| c.id == id)
    }

    pub fn send_message(&mut self, channel_id: ChannelId, message: Message) -> Result<(), Error> {
        if let Some(channel) = self.find_channel_mut(channel_id) {
            if matches!(channel.state, ChannelState::Closed) {
                return Err(Error::state("Channel is closed"));
            }
            channel.send(message);
            Ok(())
        } else {
            Err(Error::not_found("Channel not found"))
        }
    }

    pub fn receive_message(&mut self, channel_id: ChannelId) -> Result<Option<Message>, Error> {
        if let Some(channel) = self.find_channel_mut(channel_id) {
            Ok(channel.receive())
        } else {
            Err(Error::not_found("Channel not found"))
        }
    }

    pub fn close_channel(&mut self, channel_id: ChannelId) -> Result<(), Error> {
        if let Some(channel) = self.find_channel_mut(channel_id) {
            channel.close();
            Ok(())
        } else {
            Err(Error::not_found("Channel not found"))
        }
    }

    pub fn list_channels(&self) -> Vec<&Channel> {
        self.channels.iter().collect()
    }

    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }

    pub fn total_messages(&self) -> usize {
        self.channels.iter().map(|c| c.len()).sum()
    }

    pub fn clear_closed_channels(&mut self) {
        self.channels
            .retain(|c| !matches!(c.state, ChannelState::Closed));
    }
}

impl Default for IpcManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_creation() {
        let ipc = IpcManager::new();
        assert_eq!(ipc.channel_count(), 0);
    }

    #[test]
    fn test_create_channel() {
        let mut ipc = IpcManager::new();
        let id = ipc
            .create_channel("test", ProcessId(1), ProcessId(2))
            .unwrap();
        assert_eq!(id.0, 1);
        assert_eq!(ipc.channel_count(), 1);
    }

    #[test]
    fn test_send_receive() {
        let mut ipc = IpcManager::new();
        let channel_id = ipc
            .create_channel("test", ProcessId(1), ProcessId(2))
            .unwrap();

        let msg = Message::new(ProcessId(1), ProcessId(2), b"hello".to_vec());
        ipc.send_message(channel_id, msg).unwrap();

        let received = ipc.receive_message(channel_id).unwrap().unwrap();
        assert_eq!(received.payload_str(), Some("hello".to_string()));
    }

    #[test]
    fn test_channel_close() {
        let mut ipc = IpcManager::new();
        let channel_id = ipc
            .create_channel("test", ProcessId(1), ProcessId(2))
            .unwrap();
        ipc.close_channel(channel_id).unwrap();

        let msg = Message::new(ProcessId(1), ProcessId(2), b"hello".to_vec());
        assert!(ipc.send_message(channel_id, msg).is_err());
    }

    #[test]
    fn test_total_messages() {
        let mut ipc = IpcManager::new();
        let channel_id = ipc
            .create_channel("test", ProcessId(1), ProcessId(2))
            .unwrap();

        ipc.send_message(
            channel_id,
            Message::new(ProcessId(1), ProcessId(2), b"1".to_vec()),
        )
        .unwrap();
        ipc.send_message(
            channel_id,
            Message::new(ProcessId(1), ProcessId(2), b"2".to_vec()),
        )
        .unwrap();

        assert_eq!(ipc.total_messages(), 2);
    }
}
