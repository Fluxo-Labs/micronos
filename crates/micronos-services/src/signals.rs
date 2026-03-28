use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Signal {
    Hangup,
    Interrupt,
    Quit,
    Illegal,
    Trap,
    Abort,
    BusError,
    FloatingPointException,
    Kill,
    User1,
    SegFault,
    User2,
    Pipe,
    Alarm,
    Terminate,
    StackFault,
    Continue,
    Stop,
    TSTP,
    TTIN,
    TTOU,
    Urgent,
}

impl Signal {
    pub fn name(&self) -> &'static str {
        match self {
            Signal::Hangup => "SIGHUP",
            Signal::Interrupt => "SIGINT",
            Signal::Quit => "SIGQUIT",
            Signal::Illegal => "SIGILL",
            Signal::Trap => "SIGTRAP",
            Signal::Abort => "SIGABRT",
            Signal::BusError => "SIGBUS",
            Signal::FloatingPointException => "SIGFPE",
            Signal::Kill => "SIGKILL",
            Signal::User1 => "SIGUSR1",
            Signal::SegFault => "SIGSEGV",
            Signal::User2 => "SIGUSR2",
            Signal::Pipe => "SIGPIPE",
            Signal::Alarm => "SIGALRM",
            Signal::Terminate => "SIGTERM",
            Signal::StackFault => "SIGSTKFLT",
            Signal::Continue => "SIGCONT",
            Signal::Stop => "SIGSTOP",
            Signal::TSTP => "SIGTSTP",
            Signal::TTIN => "SIGTTIN",
            Signal::TTOU => "SIGTTOU",
            Signal::Urgent => "SIGURG",
        }
    }

    pub fn from_u8(val: u8) -> Option<Signal> {
        match val {
            0 => Some(Signal::Hangup),
            1 => Some(Signal::Interrupt),
            2 => Some(Signal::Quit),
            3 => Some(Signal::Illegal),
            4 => Some(Signal::Trap),
            5 => Some(Signal::Abort),
            6 => Some(Signal::BusError),
            7 => Some(Signal::FloatingPointException),
            8 => Some(Signal::Kill),
            9 => Some(Signal::User1),
            10 => Some(Signal::SegFault),
            11 => Some(Signal::User2),
            12 => Some(Signal::Pipe),
            13 => Some(Signal::Alarm),
            14 => Some(Signal::Terminate),
            15 => Some(Signal::StackFault),
            16 => Some(Signal::Continue),
            17 => Some(Signal::Stop),
            18 => Some(Signal::TSTP),
            19 => Some(Signal::TTIN),
            20 => Some(Signal::TTOU),
            21 => Some(Signal::Urgent),
            _ => None,
        }
    }
}

pub type SignalHandler = fn(Signal);

struct HandlerEntry {
    signal: Signal,
    handler: SignalHandler,
}

pub struct SignalManager {
    handlers: Vec<HandlerEntry>,
    pending_signals: Vec<Signal>,
}

impl SignalManager {
    pub fn new() -> Self {
        SignalManager {
            handlers: Vec::new(),
            pending_signals: Vec::new(),
        }
    }

    pub fn register(&mut self, signal: Signal, handler: SignalHandler) {
        if let Some(entry) = self.handlers.iter_mut().find(|e| e.signal == signal) {
            entry.handler = handler;
        } else {
            self.handlers.push(HandlerEntry { signal, handler });
        }
    }

    pub fn unregister(&mut self, signal: Signal) {
        self.handlers.retain(|e| e.signal != signal);
    }

    pub fn send_signal(&mut self, signal: Signal) {
        self.pending_signals.push(signal);
    }

    pub fn process_signals(&mut self) -> Vec<Signal> {
        let mut processed = Vec::new();
        while let Some(signal) = self.pending_signals.pop() {
            if let Some(entry) = self.handlers.iter().find(|e| e.signal == signal) {
                (entry.handler)(signal);
            }
            processed.push(signal);
        }
        processed
    }

    pub fn pending_count(&self) -> usize {
        self.pending_signals.len()
    }

    pub fn clear_pending(&mut self) {
        self.pending_signals.clear();
    }

    pub fn has_handler(&self, signal: Signal) -> bool {
        self.handlers.iter().any(|e| e.signal == signal)
    }

    pub fn list_signals(&self) -> Vec<Signal> {
        self.handlers.iter().map(|e| e.signal).collect()
    }
}

impl Default for SignalManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_creation() {
        let signal = Signal::Interrupt;
        assert_eq!(signal.name(), "SIGINT");
    }

    #[test]
    fn test_signal_from_u8() {
        assert_eq!(Signal::from_u8(1), Some(Signal::Interrupt));
        assert_eq!(Signal::from_u8(8), Some(Signal::Kill));
        assert_eq!(Signal::from_u8(99), None);
    }

    #[test]
    fn test_signal_manager() {
        let mut sm = SignalManager::new();
        assert_eq!(sm.pending_count(), 0);
        sm.send_signal(Signal::Interrupt);
        assert_eq!(sm.pending_count(), 1);
    }
}
