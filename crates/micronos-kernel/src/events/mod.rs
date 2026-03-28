//! Event system for MicronOS kernel
//!
//! Provides a typed event system for inter-component communication.

extern crate spin;

use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::RwLock;

static NEXT_EVENT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    ProcessCreated,
    ProcessTerminated,
    ProcessSuspended,
    ProcessResumed,
    TimerExpired,
    SignalReceived,
    NetworkConnected,
    NetworkDisconnected,
    DiskRead,
    DiskWrite,
    MemoryAllocated,
    MemoryFreed,
    Custom(u32),
}

impl EventType {
    pub fn as_u32(&self) -> u32 {
        match self {
            EventType::ProcessCreated => 0,
            EventType::ProcessTerminated => 1,
            EventType::ProcessSuspended => 2,
            EventType::ProcessResumed => 3,
            EventType::TimerExpired => 4,
            EventType::SignalReceived => 5,
            EventType::NetworkConnected => 6,
            EventType::NetworkDisconnected => 7,
            EventType::DiskRead => 8,
            EventType::DiskWrite => 9,
            EventType::MemoryAllocated => 10,
            EventType::MemoryFreed => 11,
            EventType::Custom(id) => *id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub id: u64,
    pub event_type: EventType,
    pub timestamp: u64,
    pub source: EventSource,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSource {
    Kernel,
    Process(u32),
    Driver(u32),
    Network,
    Timer,
    Custom(u32),
}

impl Event {
    pub fn new(event_type: EventType, source: EventSource, timestamp: u64) -> Self {
        Event {
            id: NEXT_EVENT_ID.fetch_add(1, Ordering::SeqCst),
            event_type,
            timestamp,
            source,
            data: Vec::new(),
        }
    }

    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
}

pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &Event);
    fn event_types(&self) -> Vec<EventType>;
}

pub struct EventBus {
    handlers: RwLock<Vec<Arc<dyn EventHandler>>>,
    history: RwLock<Vec<Event>>,
    max_history: usize,
}

impl EventBus {
    pub fn new(max_history: usize) -> Self {
        EventBus {
            handlers: RwLock::new(Vec::new()),
            history: RwLock::new(Vec::new()),
            max_history,
        }
    }

    pub fn subscribe(&self, handler: Arc<dyn EventHandler>) {
        self.handlers.write().push(handler);
    }

    pub fn unsubscribe(&self, handler: Arc<dyn EventHandler>) {
        self.handlers.write().retain(|h| !Arc::ptr_eq(h, &handler));
    }

    pub fn publish(&self, event: Event) {
        for handler in self.handlers.read().iter() {
            if handler.event_types().contains(&event.event_type) {
                handler.handle(&event);
            }
        }

        let mut history = self.history.write();
        history.push(event);
        if history.len() > self.max_history {
            history.remove(0);
        }
    }

    pub fn history(&self) -> Vec<Event> {
        self.history.read().clone()
    }

    pub fn history_by_type(&self, event_type: EventType) -> Vec<Event> {
        self.history
            .read()
            .iter()
            .filter(|e| e.event_type == event_type)
            .cloned()
            .collect()
    }

    pub fn clear_history(&self) {
        self.history.write().clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1000)
    }
}

pub struct SystemEventHandler {
    event_counts: RwLock<Vec<(EventType, u64)>>,
}

impl SystemEventHandler {
    pub fn new() -> Self {
        SystemEventHandler {
            event_counts: RwLock::new(Vec::new()),
        }
    }

    pub fn get_counts(&self) -> Vec<(EventType, u64)> {
        self.event_counts.read().clone()
    }
}

impl Clone for SystemEventHandler {
    fn clone(&self) -> Self {
        SystemEventHandler {
            event_counts: RwLock::new(self.event_counts.read().clone()),
        }
    }
}

impl Default for SystemEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for SystemEventHandler {
    fn handle(&self, event: &Event) {
        let mut counts = self.event_counts.write();
        if let Some(pos) = counts.iter().position(|(t, _)| *t == event.event_type) {
            counts[pos].1 += 1;
        } else {
            counts.push((event.event_type, 1));
        }
    }

    fn event_types(&self) -> Vec<EventType> {
        use alloc::vec;
        vec![
            EventType::ProcessCreated,
            EventType::ProcessTerminated,
            EventType::TimerExpired,
            EventType::MemoryAllocated,
            EventType::MemoryFreed,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler {
        received: RwLock<Vec<Event>>,
    }

    impl TestHandler {
        fn new() -> Self {
            TestHandler {
                received: RwLock::new(Vec::new()),
            }
        }
    }

    impl EventHandler for TestHandler {
        fn handle(&self, event: &Event) {
            self.received.write().push(event.clone());
        }

        fn event_types(&self) -> Vec<EventType> {
            use alloc::vec;
            vec![EventType::ProcessCreated, EventType::TimerExpired]
        }
    }

    #[test]
    fn test_event_creation() {
        let event = Event::new(EventType::ProcessCreated, EventSource::Kernel, 1000);
        assert_eq!(event.timestamp, 1000);
        assert!(event.data.is_empty());
    }

    #[test]
    fn test_event_with_data() {
        let event = Event::new(EventType::MemoryAllocated, EventSource::Kernel, 2000)
            .with_data(alloc::vec![1, 2, 3, 4]);

        assert_eq!(event.data, alloc::vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_event_bus_subscribe() {
        let bus = EventBus::new(10);
        let handler = Arc::new(TestHandler::new());

        bus.subscribe(handler.clone());

        let handlers = bus.handlers.read();
        assert_eq!(handlers.len(), 1);
    }

    #[test]
    fn test_event_bus_publish() {
        let bus = EventBus::new(10);
        let handler = Arc::new(TestHandler::new());

        bus.subscribe(handler.clone());

        let event = Event::new(EventType::ProcessCreated, EventSource::Kernel, 3000);
        bus.publish(event);

        let received = handler.received.read();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].timestamp, 3000);
    }

    #[test]
    fn test_event_bus_history() {
        let bus = EventBus::new(3);

        bus.publish(Event::new(EventType::TimerExpired, EventSource::Kernel, 1));
        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            2,
        ));
        bus.publish(Event::new(EventType::TimerExpired, EventSource::Kernel, 3));
        bus.publish(Event::new(
            EventType::ProcessTerminated,
            EventSource::Kernel,
            4,
        ));

        let history = bus.history();
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_event_type_as_u32() {
        assert_eq!(EventType::ProcessCreated.as_u32(), 0);
        assert_eq!(EventType::TimerExpired.as_u32(), 4);
        assert_eq!(EventType::Custom(100).as_u32(), 100);
    }

    #[test]
    fn test_system_event_handler() {
        let handler = Arc::new(SystemEventHandler::new());
        let handler_for_check = Arc::clone(&handler);
        let bus = EventBus::new(10);
        bus.subscribe(handler);

        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            1,
        ));
        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            2,
        ));
        bus.publish(Event::new(EventType::TimerExpired, EventSource::Kernel, 3));

        let counts = handler_for_check.get_counts();
        assert_eq!(counts.len(), 2);
    }
}
