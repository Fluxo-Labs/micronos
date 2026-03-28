use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
    Expired,
}

#[derive(Debug, Clone)]
pub struct Timer {
    pub id: TimerId,
    pub name: String,
    pub duration_ms: u64,
    pub remaining_ms: u64,
    pub state: TimerState,
    pub callback: Option<String>,
}

impl Timer {
    pub fn new(id: TimerId, name: &str, duration_ms: u64) -> Self {
        Timer {
            id,
            name: name.to_string(),
            duration_ms,
            remaining_ms: duration_ms,
            state: TimerState::Idle,
            callback: None,
        }
    }

    pub fn start(&mut self) {
        if matches!(self.state, TimerState::Idle | TimerState::Paused) {
            self.state = TimerState::Running;
        }
    }

    pub fn pause(&mut self) {
        if matches!(self.state, TimerState::Running) {
            self.state = TimerState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if matches!(self.state, TimerState::Paused) {
            self.state = TimerState::Running;
        }
    }

    pub fn stop(&mut self) {
        self.state = TimerState::Idle;
        self.remaining_ms = self.duration_ms;
    }

    pub fn tick(&mut self, delta_ms: u64) -> bool {
        if matches!(self.state, TimerState::Running) {
            if self.remaining_ms > delta_ms {
                self.remaining_ms -= delta_ms;
                false
            } else {
                self.remaining_ms = 0;
                self.state = TimerState::Expired;
                true
            }
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.remaining_ms = self.duration_ms;
        self.state = TimerState::Idle;
    }

    pub fn set_callback(&mut self, callback: &str) {
        self.callback = Some(callback.to_string());
    }

    pub fn progress_percent(&self) -> f64 {
        if self.duration_ms == 0 {
            100.0
        } else {
            ((self.duration_ms - self.remaining_ms) as f64 / self.duration_ms as f64) * 100.0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimerId(pub u64);

pub struct TimerManager {
    timers: Vec<Timer>,
    next_id: u64,
    tick_count: u64,
}

impl TimerManager {
    pub fn new() -> Self {
        TimerManager {
            timers: Vec::new(),
            next_id: 1,
            tick_count: 0,
        }
    }

    pub fn create_timer(&mut self, name: &str, duration_ms: u64) -> TimerId {
        let id = TimerId(self.next_id);
        self.next_id += 1;
        let timer = Timer::new(id, name, duration_ms);
        self.timers.push(timer);
        id
    }

    pub fn start_timer(&mut self, id: TimerId) -> Option<&mut Timer> {
        if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
            timer.start();
            Some(timer)
        } else {
            None
        }
    }

    pub fn stop_timer(&mut self, id: TimerId) -> Option<&mut Timer> {
        if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
            timer.stop();
            Some(timer)
        } else {
            None
        }
    }

    pub fn pause_timer(&mut self, id: TimerId) -> Option<&mut Timer> {
        if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
            timer.pause();
            Some(timer)
        } else {
            None
        }
    }

    pub fn resume_timer(&mut self, id: TimerId) -> Option<&mut Timer> {
        if let Some(timer) = self.timers.iter_mut().find(|t| t.id == id) {
            timer.resume();
            Some(timer)
        } else {
            None
        }
    }

    pub fn tick(&mut self, delta_ms: u64) -> Vec<TimerId> {
        self.tick_count += 1;
        let mut expired = Vec::new();
        for timer in &mut self.timers {
            if timer.tick(delta_ms) {
                expired.push(timer.id);
            }
        }
        expired
    }

    pub fn remove_timer(&mut self, id: TimerId) -> bool {
        let len = self.timers.len();
        self.timers.retain(|t| t.id != id);
        self.timers.len() != len
    }

    pub fn clear_expired(&mut self) {
        self.timers
            .retain(|t| !matches!(t.state, TimerState::Expired));
    }

    pub fn list_timers(&self) -> Vec<&Timer> {
        self.timers.iter().collect()
    }

    pub fn timer_count(&self) -> usize {
        self.timers.len()
    }

    pub fn running_count(&self) -> usize {
        self.timers
            .iter()
            .filter(|t| matches!(t.state, TimerState::Running))
            .count()
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn get_timer(&self, id: TimerId) -> Option<&Timer> {
        self.timers.iter().find(|t| t.id == id)
    }
}

impl Default for TimerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_creation() {
        let timer = Timer::new(TimerId(1), "test", 1000);
        assert_eq!(timer.duration_ms, 1000);
        assert!(matches!(timer.state, TimerState::Idle));
    }

    #[test]
    fn test_timer_start() {
        let mut timer = Timer::new(TimerId(1), "test", 1000);
        timer.start();
        assert!(matches!(timer.state, TimerState::Running));
    }

    #[test]
    fn test_timer_tick() {
        let mut timer = Timer::new(TimerId(1), "test", 100);
        timer.start();
        let expired = timer.tick(50);
        assert!(!expired);
        assert_eq!(timer.remaining_ms, 50);
    }

    #[test]
    fn test_timer_expire() {
        let mut timer = Timer::new(TimerId(1), "test", 100);
        timer.start();
        let expired = timer.tick(100);
        assert!(expired);
        assert!(matches!(timer.state, TimerState::Expired));
    }

    #[test]
    fn test_timer_manager() {
        let mut tm = TimerManager::new();
        let id = tm.create_timer("test", 1000);
        assert_eq!(id.0, 1);
        assert_eq!(tm.timer_count(), 1);
    }
}
