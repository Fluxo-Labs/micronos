use crate::syscall::{SyscallContext, SyscallError, SyscallHandler, SyscallResult};
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};

static NEXT_PID: AtomicU64 = AtomicU64::new(1);
static NEXT_TID: AtomicU64 = AtomicU64::new(1);

pub struct ProcessHandler {
    pid: u64,
    ppid: u64,
    uid: u32,
    gid: u32,
}

impl ProcessHandler {
    pub fn new() -> Self {
        let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
        ProcessHandler {
            pid,
            ppid: 0,
            uid: 1000,
            gid: 1000,
        }
    }
}

impl Default for ProcessHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SyscallHandler for ProcessHandler {
    fn name(&self) -> &str {
        "ProcessSyscallHandler"
    }

    fn handle(&self, context: &SyscallContext) -> SyscallResult {
        let syscall_num = context.syscall_number % 1000;
        match syscall_num {
            0 => {
                let new_pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(new_pid as isize)
            }
            1 => SyscallResult::success(0),
            2 => SyscallResult::success(0),
            3 => SyscallResult::success(0),
            4 => SyscallResult::success(self.pid as isize),
            5 => SyscallResult::success(self.ppid as isize),
            6 => SyscallResult::success(self.uid as isize),
            7 => SyscallResult::success(self.gid as isize),
            8 => SyscallResult::success(0),
            9 => SyscallResult::success(0),
            10 => {
                let tid = NEXT_TID.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(tid as isize)
            }
            11 => SyscallResult::success(0),
            12 => SyscallResult::success(0),
            13 => SyscallResult::success(0),
            14 => SyscallResult::success(self.ppid as isize),
            15 => SyscallResult::success(self.uid as isize),
            16 => SyscallResult::success(self.gid as isize),
            _ => SyscallResult::error(SyscallError::ENOSYS),
        }
    }
}

pub struct MemoryHandler {
    current_break: AtomicU64,
}

impl MemoryHandler {
    pub fn new() -> Self {
        MemoryHandler {
            current_break: AtomicU64::new(0x10000),
        }
    }
}

impl Default for MemoryHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SyscallHandler for MemoryHandler {
    fn name(&self) -> &str {
        "MemorySyscallHandler"
    }

    fn handle(&self, context: &SyscallContext) -> SyscallResult {
        let syscall_num = context.syscall_number % 1000;
        match syscall_num {
            0 => {
                let _ = context.arg0;
                let brk = self.current_break.load(Ordering::SeqCst);
                SyscallResult::success(brk as isize)
            }
            1 => {
                let _ = context.arg0;
                let len = context.arg1 as u64;
                let _ = context.arg2;
                if len > 0 {
                    let new_addr = self.current_break.load(Ordering::SeqCst);
                    let _ = new_addr.saturating_add(len);
                    SyscallResult::success(new_addr as isize)
                } else {
                    SyscallResult::error(SyscallError::EINVAL)
                }
            }
            2 => {
                let _ = context.arg0;
                let len = context.arg1;
                if len > 0 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EINVAL)
                }
            }
            3 => SyscallResult::success(0),
            4 => SyscallResult::success(0),
            5 => SyscallResult::success(0),
            6 => SyscallResult::success(0),
            7 => SyscallResult::success(0),
            8 => SyscallResult::success(0),
            9 => SyscallResult::success(0),
            10..=13 => SyscallResult::success(0),
            _ => SyscallResult::error(SyscallError::ENOSYS),
        }
    }
}

pub struct FileHandler {
    next_fd: Arc<AtomicU64>,
}

impl FileHandler {
    pub fn new() -> Self {
        FileHandler {
            next_fd: Arc::new(AtomicU64::new(3)),
        }
    }
}

impl Default for FileHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SyscallHandler for FileHandler {
    fn name(&self) -> &str {
        "FileSyscallHandler"
    }

    fn handle(&self, context: &SyscallContext) -> SyscallResult {
        let syscall_num = context.syscall_number % 1000;
        match syscall_num {
            0 => {
                let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(fd as isize)
            }
            1 => {
                let fd = context.arg0;
                if fd < 100 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            2 => {
                let fd = context.arg0;
                if fd < 100 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            3 => {
                let fd = context.arg0;
                if fd < 100 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            4 => {
                let fd = context.arg0;
                if fd < 100 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            5..=7 => SyscallResult::success(0),
            8 => SyscallResult::success(0),
            9..=10 => {
                let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(fd as isize)
            }
            11 => SyscallResult::success(0),
            12..=15 => SyscallResult::success(0),
            16 | 17 => SyscallResult::success(0),
            18..=23 => SyscallResult::success(0),
            _ => SyscallResult::error(SyscallError::ENOSYS),
        }
    }
}

pub struct NetworkHandler {
    next_socket: Arc<AtomicU64>,
}

impl NetworkHandler {
    pub fn new() -> Self {
        NetworkHandler {
            next_socket: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl Default for NetworkHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SyscallHandler for NetworkHandler {
    fn name(&self) -> &str {
        "NetworkSyscallHandler"
    }

    fn handle(&self, context: &SyscallContext) -> SyscallResult {
        let syscall_num = context.syscall_number % 1000;
        match syscall_num {
            0 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                let socket_id = self.next_socket.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(socket_id as isize)
            }
            1 => {
                let sockfd = context.arg0;
                if sockfd < 1000 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            2 => {
                let sockfd = context.arg0;
                if sockfd < 1000 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            3 => {
                let sockfd = context.arg0;
                let _ = context.arg1;
                if sockfd < 1000 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            4 => {
                let sockfd = context.arg0;
                if sockfd < 1000 {
                    let new_fd = self.next_socket.fetch_add(1, Ordering::SeqCst);
                    SyscallResult::success(new_fd as isize)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            5 | 6 => SyscallResult::success(0),
            7 => {
                let _ = self.next_socket.fetch_add(1, Ordering::SeqCst);
                let _ = self.next_socket.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(0)
            }
            8 => {
                let sockfd = context.arg0;
                if sockfd < 1000 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            9 => {
                let sockfd = context.arg0;
                if sockfd < 1000 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            10 | 11 => {
                let sockfd = context.arg0;
                if sockfd < 1000 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            12 => {
                let sockfd = context.arg0;
                let _ = context.arg1;
                if sockfd < 1000 {
                    SyscallResult::success(0)
                } else {
                    SyscallResult::error(SyscallError::EBADF)
                }
            }
            13 | 14 => SyscallResult::success(0),
            15..=19 => SyscallResult::success(0),
            _ => SyscallResult::error(SyscallError::ENOSYS),
        }
    }
}

pub struct SignalHandler {
    #[allow(dead_code)]
    next_handler: AtomicU64,
}

impl SignalHandler {
    pub fn new() -> Self {
        SignalHandler {
            next_handler: AtomicU64::new(1),
        }
    }
}

impl Default for SignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SyscallHandler for SignalHandler {
    fn name(&self) -> &str {
        "SignalSyscallHandler"
    }

    fn handle(&self, context: &SyscallContext) -> SyscallResult {
        let syscall_num = context.syscall_number % 1000;
        match syscall_num {
            0 => {
                let _ = context.arg0;
                let handler = context.arg1;
                SyscallResult::success(handler as isize)
            }
            1 => {
                let _ = context.arg0;
                let _ = context.arg1;
                SyscallResult::success(0)
            }
            2 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                SyscallResult::success(0)
            }
            3..=5 => SyscallResult::success(0),
            6 => {
                let _ = context.arg0;
                let handler = context.arg1;
                SyscallResult::success(handler as isize)
            }
            7 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                SyscallResult::success(0)
            }
            8..=13 => SyscallResult::success(0),
            _ => SyscallResult::error(SyscallError::ENOSYS),
        }
    }
}

pub struct TimeHandler {
    #[allow(dead_code)]
    boot_time: u64,
}

impl TimeHandler {
    pub fn new() -> Self {
        TimeHandler { boot_time: 0 }
    }
}

impl Default for TimeHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SyscallHandler for TimeHandler {
    fn name(&self) -> &str {
        "TimeSyscallHandler"
    }

    fn handle(&self, context: &SyscallContext) -> SyscallResult {
        let syscall_num = context.syscall_number % 1000;
        match syscall_num {
            0 => {
                let now = 1700000000u64;
                SyscallResult::success(now as isize)
            }
            1 => SyscallResult::success(0),
            2 => SyscallResult::success(0),
            3 => {
                let now = 1700000000u64;
                SyscallResult::success(now as isize)
            }
            4 => SyscallResult::success(0),
            5 => {
                let _ = context.arg0;
                SyscallResult::success(0)
            }
            6..=9 => SyscallResult::success(0),
            10 | 11 => {
                let _ = context.arg0;
                let _ = context.arg1;
                SyscallResult::success(0)
            }
            12..=15 => SyscallResult::success(0),
            _ => SyscallResult::error(SyscallError::ENOSYS),
        }
    }
}

pub struct IpcHandler {
    next_id: Arc<AtomicU64>,
}

impl IpcHandler {
    pub fn new() -> Self {
        IpcHandler {
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl Default for IpcHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SyscallHandler for IpcHandler {
    fn name(&self) -> &str {
        "IpcSyscallHandler"
    }

    fn handle(&self, context: &SyscallContext) -> SyscallResult {
        let syscall_num = context.syscall_number % 1000;
        match syscall_num {
            0 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let id = self.next_id.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(id as isize)
            }
            1 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                SyscallResult::success(0)
            }
            2 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                let _ = context.arg3;
                let _ = context.arg4;
                SyscallResult::success(0)
            }
            3 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                let _ = context.arg3;
                SyscallResult::success(0)
            }
            4 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                let id = self.next_id.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(id as isize)
            }
            5 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                SyscallResult::success(0)
            }
            6 | 7 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                SyscallResult::success(0)
            }
            8 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                let id = self.next_id.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(id as isize)
            }
            9 => {
                let _ = context.arg0;
                let _ = context.arg1;
                let _ = context.arg2;
                SyscallResult::success(0)
            }
            10 => {
                let _ = context.arg0;
                let shmaddr = context.arg1;
                let _ = context.arg2;
                SyscallResult::success(shmaddr as isize)
            }
            11 => {
                let _ = context.arg0;
                SyscallResult::success(0)
            }
            12 | 13 => {
                let id = self.next_id.fetch_add(1, Ordering::SeqCst);
                SyscallResult::success(id as isize)
            }
            _ => SyscallResult::error(SyscallError::ENOSYS),
        }
    }
}
