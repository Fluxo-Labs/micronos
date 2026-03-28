use alloc::format;
use alloc::string::{String, ToString};

pub mod handlers;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyscallNumber {
    Process(usize),
    Memory(i32),
    File(i32),
    Network(i32),
    Signal(i32),
    Time(i32),
    IPC(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessSyscall {
    Fork = 0,
    Exec = 1,
    Exit = 2,
    Wait = 3,
    GetPid = 4,
    GetPpid = 5,
    GetUid = 6,
    GetGid = 7,
    SetUid = 8,
    SetGid = 9,
    GetTid = 10,
    ExitGroup = 11,
    WaitPid = 12,
    Prctl = 13,
    Getppid = 14,
    Geteuid = 15,
    Getegid = 16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemorySyscall {
    Brk = 0,
    Mmap = 1,
    Munmap = 2,
    Mprotect = 3,
    Mremap = 4,
    Msync = 5,
    Mincore = 6,
    Madvise = 7,
    Mlock = 8,
    Munlock = 9,
    Shmget = 10,
    Shmat = 11,
    Shmdt = 12,
    Shmctl = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSyscall {
    Open = 0,
    Close = 1,
    Read = 2,
    Write = 3,
    Lseek = 4,
    Stat = 5,
    Fstat = 6,
    Lstat = 7,
    Poll = 8,
    Lseek64 = 9,
    Mmap2 = 10,
    Ioctl = 11,
    Fcntl = 12,
    Flock = 13,
    Fsync = 14,
    Fdatasync = 15,
    Truncate = 16,
    Ftruncate = 17,
    Getdents = 18,
    Getdents64 = 19,
    Select = 20,
    Fselect = 21,
    Truncate64 = 22,
    Ftruncate64 = 23,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkSyscall {
    Socket = 0,
    Bind = 1,
    Connect = 2,
    Listen = 3,
    Accept = 4,
    Getsockname = 5,
    Getpeername = 6,
    Socketpair = 7,
    Send = 8,
    Recv = 9,
    Sendto = 10,
    Recvfrom = 11,
    Shutdown = 12,
    Setsockopt = 13,
    Getsockopt = 14,
    Sendmsg = 15,
    Recvmsg = 16,
    Accept4 = 17,
    Recvmmsg = 18,
    Sendmmsg = 19,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSyscall {
    Signal = 0,
    Sigaction = 1,
    Sigprocmask = 2,
    Sigpending = 3,
    Sigsuspend = 4,
    Sigaltstack = 5,
    RtSignal = 6,
    RtSigaction = 7,
    RtSigprocmask = 8,
    RtSigpending = 9,
    RtSigsuspend = 10,
    RtSigreturn = 11,
    Sigaction64 = 12,
    Signal64 = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSyscall {
    Time = 0,
    Gettimeofday = 1,
    Stime = 2,
    Time64 = 3,
    Gettimeofday64 = 4,
    ClockGettime = 5,
    ClockGettime64 = 6,
    ClockSettime = 7,
    ClockSettime64 = 8,
    ClockGetres = 9,
    ClockNanosleep = 10,
    Nanosleep = 11,
    Alarm = 12,
    Getitimer = 13,
    Setitimer = 14,
    Times = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcSyscall {
    Msgget = 0,
    Msgctl = 1,
    Msgrcv = 2,
    Msgsnd = 3,
    Semget = 4,
    Semctl = 5,
    Semop = 6,
    Semtimedop = 7,
    Shmget = 8,
    Shmctl = 9,
    Shmat = 10,
    Shmdt = 11,
    Msgget64 = 12,
    Semget64 = 13,
}

impl ProcessSyscall {
    pub fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(ProcessSyscall::Fork),
            1 => Some(ProcessSyscall::Exec),
            2 => Some(ProcessSyscall::Exit),
            3 => Some(ProcessSyscall::Wait),
            4 => Some(ProcessSyscall::GetPid),
            5 => Some(ProcessSyscall::GetPpid),
            6 => Some(ProcessSyscall::GetUid),
            7 => Some(ProcessSyscall::GetGid),
            8 => Some(ProcessSyscall::SetUid),
            9 => Some(ProcessSyscall::SetGid),
            10 => Some(ProcessSyscall::GetTid),
            11 => Some(ProcessSyscall::ExitGroup),
            12 => Some(ProcessSyscall::WaitPid),
            13 => Some(ProcessSyscall::Prctl),
            14 => Some(ProcessSyscall::Getppid),
            15 => Some(ProcessSyscall::Geteuid),
            16 => Some(ProcessSyscall::Getegid),
            _ => None,
        }
    }
}

impl MemorySyscall {
    pub fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(MemorySyscall::Brk),
            1 => Some(MemorySyscall::Mmap),
            2 => Some(MemorySyscall::Munmap),
            3 => Some(MemorySyscall::Mprotect),
            4 => Some(MemorySyscall::Mremap),
            5 => Some(MemorySyscall::Msync),
            6 => Some(MemorySyscall::Mincore),
            7 => Some(MemorySyscall::Madvise),
            8 => Some(MemorySyscall::Mlock),
            9 => Some(MemorySyscall::Munlock),
            10 => Some(MemorySyscall::Shmget),
            11 => Some(MemorySyscall::Shmat),
            12 => Some(MemorySyscall::Shmdt),
            13 => Some(MemorySyscall::Shmctl),
            _ => None,
        }
    }
}

impl FileSyscall {
    pub fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(FileSyscall::Open),
            1 => Some(FileSyscall::Close),
            2 => Some(FileSyscall::Read),
            3 => Some(FileSyscall::Write),
            4 => Some(FileSyscall::Lseek),
            5 => Some(FileSyscall::Stat),
            6 => Some(FileSyscall::Fstat),
            7 => Some(FileSyscall::Lstat),
            8 => Some(FileSyscall::Poll),
            9 => Some(FileSyscall::Lseek64),
            10 => Some(FileSyscall::Mmap2),
            11 => Some(FileSyscall::Ioctl),
            12 => Some(FileSyscall::Fcntl),
            13 => Some(FileSyscall::Flock),
            14 => Some(FileSyscall::Fsync),
            15 => Some(FileSyscall::Fdatasync),
            16 => Some(FileSyscall::Truncate),
            17 => Some(FileSyscall::Ftruncate),
            18 => Some(FileSyscall::Getdents),
            19 => Some(FileSyscall::Getdents64),
            20 => Some(FileSyscall::Select),
            21 => Some(FileSyscall::Fselect),
            22 => Some(FileSyscall::Truncate64),
            23 => Some(FileSyscall::Ftruncate64),
            _ => None,
        }
    }
}

impl NetworkSyscall {
    pub fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(NetworkSyscall::Socket),
            1 => Some(NetworkSyscall::Bind),
            2 => Some(NetworkSyscall::Connect),
            3 => Some(NetworkSyscall::Listen),
            4 => Some(NetworkSyscall::Accept),
            5 => Some(NetworkSyscall::Getsockname),
            6 => Some(NetworkSyscall::Getpeername),
            7 => Some(NetworkSyscall::Socketpair),
            8 => Some(NetworkSyscall::Send),
            9 => Some(NetworkSyscall::Recv),
            10 => Some(NetworkSyscall::Sendto),
            11 => Some(NetworkSyscall::Recvfrom),
            12 => Some(NetworkSyscall::Shutdown),
            13 => Some(NetworkSyscall::Setsockopt),
            14 => Some(NetworkSyscall::Getsockopt),
            15 => Some(NetworkSyscall::Sendmsg),
            16 => Some(NetworkSyscall::Recvmsg),
            17 => Some(NetworkSyscall::Accept4),
            18 => Some(NetworkSyscall::Recvmmsg),
            19 => Some(NetworkSyscall::Sendmmsg),
            _ => None,
        }
    }
}

impl SignalSyscall {
    pub fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(SignalSyscall::Signal),
            1 => Some(SignalSyscall::Sigaction),
            2 => Some(SignalSyscall::Sigprocmask),
            3 => Some(SignalSyscall::Sigpending),
            4 => Some(SignalSyscall::Sigsuspend),
            5 => Some(SignalSyscall::Sigaltstack),
            6 => Some(SignalSyscall::RtSignal),
            7 => Some(SignalSyscall::RtSigaction),
            8 => Some(SignalSyscall::RtSigprocmask),
            9 => Some(SignalSyscall::RtSigpending),
            10 => Some(SignalSyscall::RtSigsuspend),
            11 => Some(SignalSyscall::RtSigreturn),
            12 => Some(SignalSyscall::Sigaction64),
            13 => Some(SignalSyscall::Signal64),
            _ => None,
        }
    }
}

impl TimeSyscall {
    pub fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(TimeSyscall::Time),
            1 => Some(TimeSyscall::Gettimeofday),
            2 => Some(TimeSyscall::Stime),
            3 => Some(TimeSyscall::Time64),
            4 => Some(TimeSyscall::Gettimeofday64),
            5 => Some(TimeSyscall::ClockGettime),
            6 => Some(TimeSyscall::ClockGettime64),
            7 => Some(TimeSyscall::ClockSettime),
            8 => Some(TimeSyscall::ClockSettime64),
            9 => Some(TimeSyscall::ClockGetres),
            10 => Some(TimeSyscall::ClockNanosleep),
            11 => Some(TimeSyscall::Nanosleep),
            12 => Some(TimeSyscall::Alarm),
            13 => Some(TimeSyscall::Getitimer),
            14 => Some(TimeSyscall::Setitimer),
            15 => Some(TimeSyscall::Times),
            _ => None,
        }
    }
}

impl IpcSyscall {
    pub fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(IpcSyscall::Msgget),
            1 => Some(IpcSyscall::Msgctl),
            2 => Some(IpcSyscall::Msgrcv),
            3 => Some(IpcSyscall::Msgsnd),
            4 => Some(IpcSyscall::Semget),
            5 => Some(IpcSyscall::Semctl),
            6 => Some(IpcSyscall::Semop),
            7 => Some(IpcSyscall::Semtimedop),
            8 => Some(IpcSyscall::Shmget),
            9 => Some(IpcSyscall::Shmctl),
            10 => Some(IpcSyscall::Shmat),
            11 => Some(IpcSyscall::Shmdt),
            12 => Some(IpcSyscall::Msgget64),
            13 => Some(IpcSyscall::Semget64),
            _ => None,
        }
    }
}

pub struct SyscallContext {
    pub syscall_number: usize,
    pub arg0: usize,
    pub arg1: usize,
    pub arg2: usize,
    pub arg3: usize,
    pub arg4: usize,
    pub arg5: usize,
}

impl SyscallContext {
    pub fn new(
        syscall_number: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> Self {
        SyscallContext {
            syscall_number,
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        }
    }

    pub fn get_arg(&self, index: usize) -> usize {
        match index {
            0 => self.arg0,
            1 => self.arg1,
            2 => self.arg2,
            3 => self.arg3,
            4 => self.arg4,
            5 => self.arg5,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyscallResult {
    pub value: isize,
    pub error: Option<SyscallError>,
}

impl SyscallResult {
    pub fn success(value: isize) -> Self {
        SyscallResult { value, error: None }
    }

    pub fn error(err: SyscallError) -> Self {
        SyscallResult {
            value: -1,
            error: Some(err),
        }
    }

    pub fn is_success(&self) -> bool {
        self.value >= 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallError {
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    ENOTBLK = 15,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
    EDEADLK = 35,
    ENAMETOOLONG = 36,
    ENOLCK = 37,
    ENOSYS = 38,
    ENOTEMPTY = 39,
    ELOOP = 40,
    EWOULDBLOCK = 41,
    ENOMSG = 42,
    EIDRM = 43,
    ECHRNG = 44,
    EL2NSYNC = 45,
    EL3HLT = 46,
    EL3RST = 47,
    ELNRNG = 48,
    EUNATCH = 49,
    ENOCSI = 50,
    EL2HLT = 51,
    EBADE = 52,
    EBADR = 53,
    EXFULL = 54,
    ENOANO = 55,
    EBADRQC = 56,
    EBADSLT = 57,
    EBFONT = 59,
    ENOSTR = 60,
    ENODATA = 61,
    ETIME = 62,
    ENOSR = 63,
    ENONET = 64,
    ENOPKG = 65,
    EREMOTE = 66,
    ENOLINK = 67,
    EADV = 68,
    ESRMNT = 69,
    ECOMM = 70,
    EPROTO = 71,
    EMULTIHOP = 72,
    EDOTDOT = 73,
    EBADMSG = 74,
    EOVERFLOW = 75,
    ENOTUNIQ = 76,
    EBADFD = 77,
    EREMCHG = 78,
    ELIBACC = 79,
    ELIBBAD = 80,
    ELIBSCN = 81,
    ELIBMAX = 82,
    ELIBEXEC = 83,
    ESYSNTFS = 84,
    ENOTSYNC = 85,
}

impl SyscallError {
    pub fn code(&self) -> isize {
        *self as isize
    }

    pub fn from_isize(value: isize) -> Option<Self> {
        match value {
            1 => Some(SyscallError::EPERM),
            2 => Some(SyscallError::ENOENT),
            3 => Some(SyscallError::ESRCH),
            4 => Some(SyscallError::EINTR),
            5 => Some(SyscallError::EIO),
            6 => Some(SyscallError::ENXIO),
            7 => Some(SyscallError::E2BIG),
            8 => Some(SyscallError::ENOEXEC),
            9 => Some(SyscallError::EBADF),
            10 => Some(SyscallError::ECHILD),
            11 => Some(SyscallError::EAGAIN),
            12 => Some(SyscallError::ENOMEM),
            13 => Some(SyscallError::EACCES),
            14 => Some(SyscallError::EFAULT),
            _ => None,
        }
    }
}

impl core::fmt::Display for SyscallError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            SyscallError::EPERM => "Operation not permitted",
            SyscallError::ENOENT => "No such file or directory",
            SyscallError::ESRCH => "No such process",
            SyscallError::EINTR => "Interrupted system call",
            SyscallError::EIO => "I/O error",
            SyscallError::ENXIO => "No such device or address",
            SyscallError::E2BIG => "Argument list too long",
            SyscallError::ENOEXEC => "Exec format error",
            SyscallError::EBADF => "Bad file descriptor",
            SyscallError::ECHILD => "No child processes",
            SyscallError::EAGAIN => "Try again",
            SyscallError::ENOMEM => "Out of memory",
            SyscallError::EACCES => "Permission denied",
            SyscallError::EFAULT => "Bad address",
            _ => "Unknown error",
        };
        write!(f, "{}", name)
    }
}

pub trait SyscallHandler: Send + Sync {
    fn handle(&self, context: &SyscallContext) -> SyscallResult;

    fn name(&self) -> &str {
        "Unknown"
    }
}

#[derive(Clone)]
pub struct SyscallDispatcher {
    process_handler: Option<alloc::sync::Arc<dyn SyscallHandler>>,
    memory_handler: Option<alloc::sync::Arc<dyn SyscallHandler>>,
    file_handler: Option<alloc::sync::Arc<dyn SyscallHandler>>,
    network_handler: Option<alloc::sync::Arc<dyn SyscallHandler>>,
    signal_handler: Option<alloc::sync::Arc<dyn SyscallHandler>>,
    time_handler: Option<alloc::sync::Arc<dyn SyscallHandler>>,
    ipc_handler: Option<alloc::sync::Arc<dyn SyscallHandler>>,
}

impl SyscallDispatcher {
    pub fn new() -> Self {
        SyscallDispatcher {
            process_handler: None,
            memory_handler: None,
            file_handler: None,
            network_handler: None,
            signal_handler: None,
            time_handler: None,
            ipc_handler: None,
        }
    }

    pub fn register_process_handler(&mut self, handler: alloc::sync::Arc<dyn SyscallHandler>) {
        self.process_handler = Some(handler);
    }

    pub fn register_memory_handler(&mut self, handler: alloc::sync::Arc<dyn SyscallHandler>) {
        self.memory_handler = Some(handler);
    }

    pub fn register_file_handler(&mut self, handler: alloc::sync::Arc<dyn SyscallHandler>) {
        self.file_handler = Some(handler);
    }

    pub fn register_network_handler(&mut self, handler: alloc::sync::Arc<dyn SyscallHandler>) {
        self.network_handler = Some(handler);
    }

    pub fn register_signal_handler(&mut self, handler: alloc::sync::Arc<dyn SyscallHandler>) {
        self.signal_handler = Some(handler);
    }

    pub fn register_time_handler(&mut self, handler: alloc::sync::Arc<dyn SyscallHandler>) {
        self.time_handler = Some(handler);
    }

    pub fn register_ipc_handler(&mut self, handler: alloc::sync::Arc<dyn SyscallHandler>) {
        self.ipc_handler = Some(handler);
    }

    pub fn dispatch(&self, context: &SyscallContext) -> SyscallResult {
        let syscall_group = context.syscall_number / 1000;
        let _syscall_num = context.syscall_number % 1000;

        match syscall_group {
            0 => {
                if let Some(ref handler) = self.process_handler {
                    handler.handle(context)
                } else {
                    SyscallResult::error(SyscallError::ENOSYS)
                }
            }
            1 => {
                if let Some(ref handler) = self.memory_handler {
                    handler.handle(context)
                } else {
                    SyscallResult::error(SyscallError::ENOSYS)
                }
            }
            2 => {
                if let Some(ref handler) = self.file_handler {
                    handler.handle(context)
                } else {
                    SyscallResult::error(SyscallError::ENOSYS)
                }
            }
            3 => {
                if let Some(ref handler) = self.network_handler {
                    handler.handle(context)
                } else {
                    SyscallResult::error(SyscallError::ENOSYS)
                }
            }
            4 => {
                if let Some(ref handler) = self.signal_handler {
                    handler.handle(context)
                } else {
                    SyscallResult::error(SyscallError::ENOSYS)
                }
            }
            5 => {
                if let Some(ref handler) = self.time_handler {
                    handler.handle(context)
                } else {
                    SyscallResult::error(SyscallError::ENOSYS)
                }
            }
            6 => {
                if let Some(ref handler) = self.ipc_handler {
                    handler.handle(context)
                } else {
                    SyscallResult::error(SyscallError::ENOSYS)
                }
            }
            _ => SyscallResult::error(SyscallError::ENOSYS),
        }
    }

    pub fn syscall_name(&self, syscall_number: usize) -> String {
        let syscall_group = syscall_number / 1000;
        let syscall_num = syscall_number % 1000;

        let group_name = match syscall_group {
            0 => "process",
            1 => "memory",
            2 => "file",
            3 => "network",
            4 => "signal",
            5 => "time",
            6 => "ipc",
            _ => "unknown",
        };

        let syscall_name = match syscall_group {
            0 => format!("{:?}", ProcessSyscall::from_usize(syscall_num)),
            1 => format!("{:?}", MemorySyscall::from_usize(syscall_num)),
            2 => format!("{:?}", FileSyscall::from_usize(syscall_num)),
            3 => format!("{:?}", NetworkSyscall::from_usize(syscall_num)),
            4 => format!("{:?}", SignalSyscall::from_usize(syscall_num)),
            5 => format!("{:?}", TimeSyscall::from_usize(syscall_num)),
            6 => format!("{:?}", IpcSyscall::from_usize(syscall_num)),
            _ => "Unknown".to_string(),
        };

        format!("sys_{}_{}", group_name, syscall_name)
    }
}

impl Default for SyscallDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for SyscallDispatcher {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SyscallDispatcher")
    }
}

pub use handlers::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall_result() {
        let success = SyscallResult::success(42);
        assert!(success.is_success());
        assert_eq!(success.value, 42);

        let failure = SyscallResult::error(SyscallError::ENOENT);
        assert!(!failure.is_success());
        assert_eq!(failure.value, -1);
    }

    #[test]
    fn test_syscall_error() {
        assert_eq!(SyscallError::ENOENT.code(), 2);
        assert_eq!(SyscallError::from_isize(2), Some(SyscallError::ENOENT));
    }

    #[test]
    fn test_process_syscall_from_usize() {
        assert_eq!(ProcessSyscall::from_usize(0), Some(ProcessSyscall::Fork));
        assert_eq!(ProcessSyscall::from_usize(4), Some(ProcessSyscall::GetPid));
        assert_eq!(ProcessSyscall::from_usize(100), None);
    }

    #[test]
    fn test_file_syscall_from_usize() {
        assert_eq!(FileSyscall::from_usize(0), Some(FileSyscall::Open));
        assert_eq!(FileSyscall::from_usize(2), Some(FileSyscall::Read));
        assert_eq!(FileSyscall::from_usize(100), None);
    }

    #[test]
    fn test_syscall_context() {
        let ctx = SyscallContext::new(1000, 10, 20, 30, 40, 50, 60);
        assert_eq!(ctx.get_arg(0), 10);
        assert_eq!(ctx.get_arg(1), 20);
        assert_eq!(ctx.get_arg(5), 60);
        assert_eq!(ctx.get_arg(10), 0);
    }
}
