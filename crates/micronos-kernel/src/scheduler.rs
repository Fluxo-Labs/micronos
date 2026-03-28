extern crate alloc;
use alloc::vec::Vec;
use micronos_core::error::Result;
use micronos_core::types::{Priority, ThreadId};

#[derive(Debug, Clone, Default)]
pub enum SchedulerState {
    #[default]
    Idle,
    Active,
    Paused,
}

pub struct MicronScheduler {
    state: SchedulerState,
    threads: Vec<ThreadEntry>,
    #[allow(dead_code)]
    current_tick: u64,
}

struct ThreadEntry {
    id: ThreadId,
    priority: Priority,
    #[allow(dead_code)]
    state: ThreadStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum ThreadStatus {
    Ready,
    Running,
    Blocked,
    Terminated,
}

impl MicronScheduler {
    pub fn new() -> Self {
        MicronScheduler {
            state: SchedulerState::Idle,
            threads: Vec::new(),
            current_tick: 0,
        }
    }

    pub fn activate(&mut self) {
        if matches!(self.state, SchedulerState::Idle) {
            self.state = SchedulerState::Active;
        }
    }

    pub fn pause(&mut self) {
        if matches!(self.state, SchedulerState::Active) {
            self.state = SchedulerState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if matches!(self.state, SchedulerState::Paused) {
            self.state = SchedulerState::Active;
        }
    }

    pub fn stop(&mut self) {
        self.state = SchedulerState::Idle;
        self.threads.clear();
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, SchedulerState::Active)
    }

    pub fn schedule(&self) -> Option<ThreadId> {
        None
    }

    pub fn add_thread(&mut self, thread: ThreadId, priority: Priority) -> Result<()> {
        self.threads.push(ThreadEntry {
            id: thread,
            priority,
            state: ThreadStatus::Ready,
        });
        Ok(())
    }

    pub fn remove_thread(&mut self, thread: ThreadId) -> Result<()> {
        self.threads.retain(|t| t.id != thread);
        Ok(())
    }

    pub fn yield_current(&self) -> Result<()> {
        Ok(())
    }

    pub fn set_priority(&mut self, thread: ThreadId, priority: Priority) -> Result<()> {
        if let Some(t) = self.threads.iter_mut().find(|t| t.id == thread) {
            t.priority = priority;
        }
        Ok(())
    }
}

impl Default for MicronScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = MicronScheduler::new();
        assert!(matches!(scheduler.state, SchedulerState::Idle));
    }

    #[test]
    fn test_scheduler_activation() {
        let mut scheduler = MicronScheduler::new();
        scheduler.activate();
        assert!(matches!(scheduler.state, SchedulerState::Active));
    }

    #[test]
    fn test_scheduler_pause_resume() {
        let mut scheduler = MicronScheduler::new();
        scheduler.activate();
        scheduler.pause();
        assert!(matches!(scheduler.state, SchedulerState::Paused));
        scheduler.resume();
        assert!(matches!(scheduler.state, SchedulerState::Active));
    }

    #[test]
    fn test_add_thread() {
        let mut scheduler = MicronScheduler::new();
        let tid = ThreadId(1);
        scheduler.add_thread(tid, Priority::Normal).unwrap();
        assert_eq!(scheduler.threads.len(), 1);
    }
}
