use alloc::string::{String, ToString};
use alloc::vec::Vec;
use micronos_core::error::Error;
use micronos_core::types::{Priority, ProcessId, ThreadId};

#[derive(Debug, Clone, Default)]
pub enum ProcessState {
    #[default]
    Created,
    Ready,
    Running {
        cpu_time_ms: u64,
    },
    Blocked {
        wait_reason: WaitReason,
    },
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Default)]
pub enum PMState {
    #[default]
    Spawning,
    Active,
    Suspended,
    Terminated,
    Crashed,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: ProcessId,
    pub name: String,
    pub state: ProcessState,
    pub priority: Priority,
    pub threads: Vec<ThreadId>,
    pub memory_kb: u64,
    pub cpu_time_ms: u64,
    pub parent: Option<ProcessId>,
}

impl Process {
    pub fn new(id: ProcessId, name: &str) -> Self {
        Process {
            id,
            name: name.to_string(),
            state: ProcessState::Created,
            priority: Priority::Normal,
            threads: Vec::new(),
            memory_kb: 0,
            cpu_time_ms: 0,
            parent: None,
        }
    }

    pub fn spawn(&mut self) {
        if matches!(self.state, ProcessState::Created) {
            self.state = ProcessState::Ready;
        }
    }

    pub fn run(&mut self) {
        if matches!(self.state, ProcessState::Ready) {
            self.state = ProcessState::Running { cpu_time_ms: 0 };
        }
    }

    pub fn block(&mut self, reason: WaitReason) {
        if matches!(self.state, ProcessState::Running { .. }) {
            self.state = ProcessState::Blocked {
                wait_reason: reason,
            };
        }
    }

    pub fn terminate(&mut self) {
        self.state = ProcessState::Terminated;
    }
}

pub struct ProcessManager {
    pub state: PMState,
    pub processes: Vec<Process>,
    pub next_pid: u64,
    pub max_processes: u32,
}

impl ProcessManager {
    pub const MAX_PROCESSES: u32 = 1024;

    pub fn new() -> Self {
        let mut pm = ProcessManager {
            state: PMState::Spawning,
            processes: Vec::new(),
            next_pid: 1,
            max_processes: Self::MAX_PROCESSES,
        };
        pm.spawn_system_processes();
        pm
    }

    fn spawn_system_processes(&mut self) {
        let system_procs = ["init", "kernel", "service_mgr", "health_monitor", "logger"];
        for name in system_procs {
            let pid = ProcessId(self.next_pid);
            self.next_pid += 1;
            let mut proc = Process::new(pid, name);
            proc.run();
            self.processes.push(proc);
        }
    }

    pub fn activate(&mut self) {
        if matches!(self.state, PMState::Spawning) {
            self.state = PMState::Active;
        }
    }

    pub fn suspend_all(&mut self) {
        if matches!(self.state, PMState::Active) {
            self.state = PMState::Suspended;
        }
    }

    pub fn resume_all(&mut self) {
        if matches!(self.state, PMState::Suspended) {
            self.state = PMState::Active;
        }
    }

    pub fn cleanup(&mut self) {
        for proc in &mut self.processes {
            proc.terminate();
        }
        self.state = PMState::Terminated;
    }

    pub fn handle_crash(&mut self) {
        self.state = PMState::Crashed;
    }

    pub fn process_count(&self) -> u32 {
        self.processes.len() as u32
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, PMState::Active)
    }

    pub fn list_processes(&self) -> Vec<&Process> {
        self.processes
            .iter()
            .filter(|p| !matches!(p.state, ProcessState::Terminated))
            .collect()
    }

    pub fn find_process(&self, pid: ProcessId) -> Option<&Process> {
        self.processes.iter().find(|p| p.id == pid)
    }

    pub fn kill_process(&mut self, pid: ProcessId) -> Result<(), Error> {
        if let Some(proc) = self.processes.iter_mut().find(|p| p.id == pid) {
            if matches!(proc.state, ProcessState::Terminated) {
                return Err(Error::data("Process already terminated"));
            }
            proc.terminate();
            Ok(())
        } else {
            Err(Error::not_found("Process not found"))
        }
    }

    pub fn spawn_process(&mut self, name: &str) -> Result<ProcessId, Error> {
        if self.processes.len() >= self.max_processes as usize {
            return Err(Error::memory("Max processes reached"));
        }
        let pid = ProcessId(self.next_pid);
        self.next_pid += 1;
        let mut proc = Process::new(pid, name);
        proc.spawn();
        proc.run();
        self.processes.push(proc);
        Ok(pid)
    }

    pub fn suspend_process(&mut self, pid: ProcessId) -> Result<(), Error> {
        if let Some(proc) = self.processes.iter_mut().find(|p| p.id == pid) {
            if matches!(proc.state, ProcessState::Running { .. }) {
                proc.state = ProcessState::Suspended;
                Ok(())
            } else {
                Err(Error::InvalidState)
            }
        } else {
            Err(Error::State("Process not found".into()))
        }
    }

    pub fn resume_process(&mut self, pid: ProcessId) -> Result<(), Error> {
        if let Some(proc) = self.processes.iter_mut().find(|p| p.id == pid) {
            if matches!(proc.state, ProcessState::Suspended) {
                proc.state = ProcessState::Ready;
                Ok(())
            } else {
                Err(Error::InvalidState)
            }
        } else {
            Err(Error::State("Process not found".into()))
        }
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitReason {
    Io,
    Timer,
    Mutex,
    Channel,
    Semaphore,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pm_creation() {
        let pm = ProcessManager::new();
        assert!(matches!(pm.state, PMState::Spawning));
        assert_eq!(pm.process_count(), 5);
    }

    #[test]
    fn test_pm_activation() {
        let mut pm = ProcessManager::new();
        pm.activate();
        assert!(matches!(pm.state, PMState::Active));
    }

    #[test]
    fn test_suspend_resume() {
        let mut pm = ProcessManager::new();
        pm.activate();
        pm.suspend_all();
        assert!(matches!(pm.state, PMState::Suspended));
        pm.resume_all();
        assert!(matches!(pm.state, PMState::Active));
    }

    #[test]
    fn test_process_count() {
        let pm = ProcessManager::new();
        assert!(pm.process_count() >= 5);
    }

    #[test]
    fn test_spawn_process() {
        let mut pm = ProcessManager::new();
        let pid = pm.spawn_process("test_proc").unwrap();
        assert_eq!(pid.0, 6);
        assert_eq!(pm.process_count(), 6);
    }

    #[test]
    fn test_kill_process() {
        let mut pm = ProcessManager::new();
        let pid = pm.spawn_process("temp").unwrap();
        pm.kill_process(pid).unwrap();
        assert_eq!(pm.process_count(), 6);
    }
}
