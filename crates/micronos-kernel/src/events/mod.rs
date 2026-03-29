//! Event system for MicronOS kernel
//!
//! Provides a typed event system for inter-component communication.
//!
//! # Architecture
//!
//! The event system consists of:
//! - [`EventBus`] - Central pub/sub bus for event distribution
//! - [`Event`] - Represents a single event with type, source, timestamp, and data
//! - [`EventHandler`] - Trait for handling events
//! - [`SystemEventHandler`] - Built-in handler that tracks event counts
//!
//! # Event Types
//!
//! - Process: Created, Terminated, Suspended, Resumed
//! - Timer: Expired
//! - Network: Connected, Disconnected
//! - Memory: Allocated, Freed
//! - Disk: Read, Write
//! - Signal: Received
//! - Custom: User-defined events (Custom(id))
//!
//! # Event Sources
//!
//! - Kernel: System-level events
//! - Process(pid): Events from specific process
//! - Driver(id): Events from drivers
//! - Network: Network-related events
//! - Timer: Timer-related events
//! - Custom(id): User-defined sources
//!
//! # Usage
//!
//! ```rust,ignore
//! use micronos_kernel::events::{EventBus, Event, EventType, EventSource, SystemEventHandler};
//! use alloc::sync::Arc;
//!
//! // Create event bus with history limit
//! let bus = EventBus::new(1000);
//!
//! // Subscribe handler
//! bus.subscribe(Arc::new(SystemEventHandler::new()));
//!
//! // Publish event
//! bus.publish(Event::new(EventType::ProcessCreated, EventSource::Kernel, 100));
//!
//! // Get history
//! let history = bus.history();
//! ```

extern crate spin;

use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::RwLock;

static NEXT_EVENT_ID: AtomicU64 = AtomicU64::new(1);

/// Represents the type of an event in the system.
///
/// # Event Categories
///
/// - **Process events**: Created, Terminated, Suspended, Resumed
/// - **Timer events**: Expired
/// - **Network events**: Connected, Disconnected
/// - **Memory events**: Allocated, Freed
/// - **Disk events**: Read, Write
/// - **Signal events**: Received
/// - **Custom events**: User-defined with custom ID
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    /// A new process was created
    ProcessCreated,
    /// A process was terminated
    ProcessTerminated,
    /// A process was suspended
    ProcessSuspended,
    /// A suspended process was resumed
    ProcessResumed,
    /// A timer expired
    TimerExpired,
    /// A signal was received
    SignalReceived,
    /// Network connection established
    NetworkConnected,
    /// Network disconnected
    NetworkDisconnected,
    /// Disk read operation
    DiskRead,
    /// Disk write operation
    DiskWrite,
    /// Memory was allocated
    MemoryAllocated,
    /// Memory was freed
    MemoryFreed,
    /// Custom event with user-defined ID
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

/// Represents a single event in the system.
///
/// Contains the event type, source, timestamp, and optional data payload.
#[derive(Debug, Clone)]
pub struct Event {
    /// Unique identifier for this event
    pub id: u64,
    /// Type of the event
    pub event_type: EventType,
    /// Timestamp when the event occurred (in milliseconds)
    pub timestamp: u64,
    /// Source that generated the event
    pub source: EventSource,
    /// Optional data payload
    pub data: Vec<u8>,
}

/// Represents the source of an event.
///
/// # Variants
///
/// - **Kernel**: System-level events from the kernel
/// - **Process(pid)**: Events from a specific process
/// - **Driver(id)**: Events from drivers
/// - **Network**: Network subsystem events
/// - **Timer**: Timer subsystem events
/// - **Custom(id)**: User-defined source with custom ID
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSource {
    /// Kernel/system-level source
    Kernel,
    /// Source from a specific process (PID)
    Process(u32),
    /// Source from a specific driver (ID)
    Driver(u32),
    /// Network subsystem source
    Network,
    /// Timer subsystem source
    Timer,
    /// Custom source with user-defined ID
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

/// Central event bus for pub/sub event distribution.
///
/// # Features
///
/// - Subscribe handlers to receive specific event types
/// - Publish events to all subscribed handlers
/// - Maintain configurable event history
/// - FIFO history with automatic eviction
///
/// # Example
///
/// ```rust,ignore
/// use micronos_kernel::events::{EventBus, Event, EventType, EventSource, SystemEventHandler};
/// use alloc::sync::Arc;
///
/// let bus = EventBus::new(100);
/// bus.subscribe(Arc::new(SystemEventHandler::new()));
/// bus.publish(Event::new(EventType::ProcessCreated, EventSource::Kernel, 100));
/// ```
pub struct EventBus {
    handlers: RwLock<Vec<Arc<dyn EventHandler>>>,
    history: RwLock<Vec<Event>>,
    max_history: usize,
}

impl EventBus {
    /// Creates a new EventBus with the specified history limit.
    ///
    /// When history exceeds max_history, oldest events are removed.
    pub fn new(max_history: usize) -> Self {
        EventBus {
            handlers: RwLock::new(Vec::new()),
            history: RwLock::new(Vec::new()),
            max_history,
        }
    }

    /// Subscribe a handler to receive events.
    pub fn subscribe(&self, handler: Arc<dyn EventHandler>) {
        self.handlers.write().push(handler);
    }

    /// Unsubscribe a handler from receiving events.
    pub fn unsubscribe(&self, handler: Arc<dyn EventHandler>) {
        self.handlers.write().retain(|h| !Arc::ptr_eq(h, &handler));
    }

    /// Publish an event to all subscribed handlers.
    ///
    /// The event is also added to the history buffer.
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

    /// Returns a copy of the event history.
    pub fn history(&self) -> Vec<Event> {
        self.history.read().clone()
    }

    /// Returns events filtered by type.
    pub fn history_by_type(&self, event_type: EventType) -> Vec<Event> {
        self.history
            .read()
            .iter()
            .filter(|e| e.event_type == event_type)
            .cloned()
            .collect()
    }

    /// Returns events within a time range (inclusive).
    ///
    /// # Arguments
    ///
    /// * `start` - Start timestamp (inclusive)
    /// * `end` - End timestamp (inclusive)
    pub fn history_by_time_range(&self, start: u64, end: u64) -> Vec<Event> {
        self.history
            .read()
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .cloned()
            .collect()
    }

    /// Returns events after a timestamp.
    pub fn history_since(&self, timestamp: u64) -> Vec<Event> {
        self.history
            .read()
            .iter()
            .filter(|e| e.timestamp >= timestamp)
            .cloned()
            .collect()
    }

    /// Returns events before a timestamp.
    pub fn history_until(&self, timestamp: u64) -> Vec<Event> {
        self.history
            .read()
            .iter()
            .filter(|e| e.timestamp <= timestamp)
            .cloned()
            .collect()
    }

    /// Clears all events from history.
    pub fn clear_history(&self) {
        self.history.write().clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Built-in event handler that tracks event counts.
///
/// # Example
///
/// ```rust,ignore
/// use micronos_kernel::events::{EventBus, SystemEventHandler};
/// use alloc::sync::Arc;
///
/// let handler = SystemEventHandler::new();
/// let bus = EventBus::new(100);
/// bus.subscribe(Arc::new(handler.clone()));
/// // ... publish events ...
/// let counts = handler.get_counts();
/// ```
pub struct SystemEventHandler {
    event_counts: RwLock<Vec<(EventType, u64)>>,
}

impl SystemEventHandler {
    /// Creates a new SystemEventHandler.
    pub fn new() -> Self {
        SystemEventHandler {
            event_counts: RwLock::new(Vec::new()),
        }
    }

    /// Returns the count of each event type received.
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

    #[test]
    fn test_event_source_equality() {
        assert_eq!(EventSource::Kernel, EventSource::Kernel);
        assert_eq!(EventSource::Process(42), EventSource::Process(42));
        assert_ne!(EventSource::Process(42), EventSource::Process(43));
        assert_eq!(EventSource::Network, EventSource::Network);
        assert_eq!(EventSource::Timer, EventSource::Timer);
    }

    #[test]
    fn test_event_bus_unsubscribe() {
        let bus = EventBus::new(10);
        let handler = Arc::new(TestHandler::new());

        bus.subscribe(handler.clone());
        assert_eq!(bus.handlers.read().len(), 1);

        bus.unsubscribe(handler.clone());
        assert_eq!(bus.handlers.read().len(), 0);
    }

    #[test]
    fn test_event_bus_filter_by_type() {
        let bus = EventBus::new(10);

        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            1,
        ));
        bus.publish(Event::new(EventType::TimerExpired, EventSource::Kernel, 2));
        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            3,
        ));
        bus.publish(Event::new(
            EventType::MemoryAllocated,
            EventSource::Kernel,
            4,
        ));

        let filtered = bus.history_by_type(EventType::ProcessCreated);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_event_bus_clear_history() {
        let bus = EventBus::new(10);

        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            1,
        ));
        bus.publish(Event::new(EventType::TimerExpired, EventSource::Kernel, 2));

        assert_eq!(bus.history().len(), 2);

        bus.clear_history();

        assert_eq!(bus.history().len(), 0);
    }

    #[test]
    fn test_event_bus_default() {
        let bus = EventBus::default();
        assert_eq!(bus.history().len(), 0);
    }

    #[test]
    fn test_system_event_handler_clone() {
        let handler1 = SystemEventHandler::new();
        let handler2 = handler1.clone();

        let counts1 = handler1.get_counts();
        let counts2 = handler2.get_counts();
        assert_eq!(counts1.len(), counts2.len());
    }

    #[test]
    fn test_event_bus_history_by_time_range() {
        let bus = EventBus::new(10);

        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            100,
        ));
        bus.publish(Event::new(EventType::TimerExpired, EventSource::Kernel, 200));
        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            300,
        ));
        bus.publish(Event::new(EventType::TimerExpired, EventSource::Kernel, 400));

        let range = bus.history_by_time_range(150, 350);
        assert_eq!(range.len(), 2);
        assert_eq!(range[0].timestamp, 200);
        assert_eq!(range[1].timestamp, 300);
    }

    #[test]
    fn test_event_bus_history_since() {
        let bus = EventBus::new(10);

        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            100,
        ));
        bus.publish(Event::new(EventType::TimerExpired, EventSource::Kernel, 200));
        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            300,
        ));

        let since = bus.history_since(200);
        assert_eq!(since.len(), 2);
        assert_eq!(since[0].timestamp, 200);
        assert_eq!(since[1].timestamp, 300);
    }

    #[test]
    fn test_event_bus_history_until() {
        let bus = EventBus::new(10);

        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            100,
        ));
        bus.publish(Event::new(EventType::TimerExpired, EventSource::Kernel, 200));
        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            300,
        ));

        let until = bus.history_until(200);
        assert_eq!(until.len(), 2);
        assert_eq!(until[0].timestamp, 100);
        assert_eq!(until[1].timestamp, 200);
    }

    #[test]
    fn test_event_bus_time_range_empty() {
        let bus = EventBus::new(10);

        bus.publish(Event::new(
            EventType::ProcessCreated,
            EventSource::Kernel,
            100,
        ));

        let range = bus.history_by_time_range(200, 300);
        assert_eq!(range.len(), 0);
    }
}
