#[derive(Debug, Clone, Default)]
pub enum HealthState {
    #[default]
    Idle,
    Monitoring,
    Degraded,
    Critical,
    Recovering,
}

pub struct HealthMonitor {
    pub state: HealthState,
    pub checks: u64,
    pub failures: u64,
}

impl HealthMonitor {
    pub fn new() -> Self {
        HealthMonitor {
            state: HealthState::Idle,
            checks: 0,
            failures: 0,
        }
    }

    pub fn start_monitoring(&mut self) {
        if matches!(self.state, HealthState::Idle) {
            self.state = HealthState::Monitoring;
        }
    }

    pub fn detect_degradation(&mut self) {
        if matches!(self.state, HealthState::Monitoring) {
            self.state = HealthState::Degraded;
        }
    }

    pub fn detect_critical(&mut self) {
        if matches!(self.state, HealthState::Degraded) {
            self.state = HealthState::Critical;
        }
    }

    pub fn begin_recovery(&mut self) {
        if matches!(self.state, HealthState::Critical) {
            self.state = HealthState::Recovering;
        }
    }

    pub fn recover_health(&mut self) {
        if matches!(self.state, HealthState::Degraded) {
            self.state = HealthState::Monitoring;
        }
    }

    pub fn recovery_complete(&mut self) {
        if matches!(self.state, HealthState::Recovering) {
            self.state = HealthState::Monitoring;
        }
    }

    pub fn recovery_failed(&mut self) {
        if matches!(self.state, HealthState::Recovering) {
            self.state = HealthState::Critical;
        }
    }

    pub fn stop_monitoring(&mut self) {
        self.state = HealthState::Idle;
    }

    pub fn health_score(&self) -> u8 {
        if self.checks == 0 {
            return 100;
        }
        let ratio = 1.0 - (self.failures as f32 / self.checks as f32);
        (ratio * 100.0) as u8
    }

    pub fn is_healthy(&self) -> bool {
        matches!(self.state, HealthState::Monitoring)
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_monitor_creation() {
        let monitor = HealthMonitor::new();
        assert!(matches!(monitor.state, HealthState::Idle));
    }

    #[test]
    fn test_start_monitoring() {
        let mut monitor = HealthMonitor::new();
        monitor.start_monitoring();
        assert!(matches!(monitor.state, HealthState::Monitoring));
    }

    #[test]
    fn test_health_score_initial() {
        let monitor = HealthMonitor::new();
        assert_eq!(monitor.health_score(), 100);
    }

    #[test]
    fn test_degradation_flow() {
        let mut monitor = HealthMonitor::new();
        monitor.start_monitoring();
        monitor.detect_degradation();
        assert!(matches!(monitor.state, HealthState::Degraded));
        monitor.recover_health();
        assert!(matches!(monitor.state, HealthState::Monitoring));
    }

    #[test]
    fn test_critical_recovery_flow() {
        let mut monitor = HealthMonitor::new();
        monitor.start_monitoring();
        monitor.detect_degradation();
        monitor.detect_critical();
        monitor.begin_recovery();
        monitor.recovery_complete();
        assert!(matches!(monitor.state, HealthState::Monitoring));
    }

    #[test]
    fn test_failed_recovery() {
        let mut monitor = HealthMonitor::new();
        monitor.start_monitoring();
        monitor.detect_degradation();
        monitor.detect_critical();
        monitor.begin_recovery();
        monitor.recovery_failed();
        assert!(matches!(monitor.state, HealthState::Critical));
    }
}
