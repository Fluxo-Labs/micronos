//! POSIX Compatibility Layer for MicronOS
//!
//! This module provides a high-level interface to system calls,
//! inspired by the POSIX standard for Unix-like systems.
//!
//! # Features
//!
//! - Process management: fork, getpid, getppid, getuid, getgid
//! - File operations: open, close, read, write, lseek
//! - Network sockets: socket, bind, listen, accept
//! - Time: time
//!
//! # Example
//!
//! ```
//! use micronos_kernel::posix::{PosixLib, errno_name};
//!
//! // errno_name returns human-readable error descriptions
//! let error_msg = errno_name(2); // Returns "ENOENT (No such file or directory)"
//! ```

use crate::syscall::{SyscallContext, SyscallDispatcher, SyscallError, SyscallResult};
use alloc::sync::Arc;

/// High-level POSIX-compatible interface for MicronOS system calls.
/// Provides ergonomic wrappers around the low-level syscall interface.
pub struct PosixLib {
    dispatcher: Arc<SyscallDispatcher>,
}

impl PosixLib {
    /// Creates a new PosixLib instance.
    pub fn new(dispatcher: Arc<SyscallDispatcher>) -> Self {
        PosixLib { dispatcher }
    }

    /// Creates a new process (fork).
    /// Returns the process ID of the child in the parent, 0 in the child.
    pub fn fork(&self) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(0, 0, 0, 0, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn getpid(&self) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(4, 0, 0, 0, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn getppid(&self) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(5, 0, 0, 0, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn getuid(&self) -> Result<u32, SyscallError> {
        let ctx = SyscallContext::new(6, 0, 0, 0, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        match self.handle_result(result) {
            Ok(v) => Ok(v as u32),
            Err(e) => Err(e),
        }
    }

    pub fn getgid(&self) -> Result<u32, SyscallError> {
        let ctx = SyscallContext::new(7, 0, 0, 0, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        match self.handle_result(result) {
            Ok(v) => Ok(v as u32),
            Err(e) => Err(e),
        }
    }

    pub fn socket(
        &self,
        domain: i32,
        socket_type: i32,
        protocol: i32,
    ) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(
            3000,
            domain as usize,
            socket_type as usize,
            protocol as usize,
            0,
            0,
            0,
        );
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn bind(&self, sockfd: i32, addr: usize, addrlen: usize) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(3001, sockfd as usize, addr, addrlen, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn listen(&self, sockfd: i32, backlog: i32) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(3003, sockfd as usize, backlog as usize, 0, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn accept(&self, sockfd: i32, addr: usize, addrlen: usize) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(3004, sockfd as usize, addr, addrlen, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn time(&self) -> Result<i64, SyscallError> {
        let ctx = SyscallContext::new(5000, 0, 0, 0, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        match self.handle_result(result) {
            Ok(v) => Ok(v as i64),
            Err(e) => Err(e),
        }
    }

    fn handle_result(&self, result: SyscallResult) -> Result<i32, SyscallError> {
        if result.is_success() {
            Ok(result.value as i32)
        } else {
            Err(result.error.unwrap_or(SyscallError::ENOSYS))
        }
    }

    pub fn open(&self, pathname: usize, flags: i32, mode: i32) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(2000, pathname, flags as usize, mode as usize, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn close(&self, fd: i32) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(2001, fd as usize, 0, 0, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn read(&self, fd: i32, buf: usize, count: usize) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(2002, fd as usize, buf, count, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn write(&self, fd: i32, buf: usize, count: usize) -> Result<i32, SyscallError> {
        let ctx = SyscallContext::new(2003, fd as usize, buf, count, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        self.handle_result(result)
    }

    pub fn lseek(&self, fd: i32, offset: i64, whence: i32) -> Result<i64, SyscallError> {
        let ctx = SyscallContext::new(2004, fd as usize, offset as usize, whence as usize, 0, 0, 0);
        let result = self.dispatcher.dispatch(&ctx);
        match self.handle_result(result) {
            Ok(v) => Ok(v as i64),
            Err(e) => Err(e),
        }
    }
}

pub struct FileDescriptor {
    fd: i32,
}

impl FileDescriptor {
    pub fn new(fd: i32) -> Option<Self> {
        if fd >= 0 {
            Some(FileDescriptor { fd })
        } else {
            None
        }
    }

    pub fn fd(&self) -> i32 {
        self.fd
    }

    pub fn is_valid(&self) -> bool {
        self.fd >= 0
    }
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {}
}

#[derive(Default)]
pub struct ProcessId(u32);

impl ProcessId {
    pub fn new(id: u32) -> Self {
        ProcessId(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Default)]
pub struct ThreadId(u32);

impl ThreadId {
    pub fn new(id: u32) -> Self {
        ThreadId(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenFlags {
    ReadOnly = 0,
    WriteOnly = 1,
    ReadWrite = 2,
    Append = 1024,
    Create = 64,
    Truncate = 512,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    Read = 0o444,
    Write = 0o222,
    Execute = 0o111,
    All = 0o777,
}

/// Returns a human-readable description of an errno code.
///
/// # Example
///
/// ```
/// use micronos_kernel::posix::errno_name;
///
/// assert!(errno_name(2).contains("No such file"));
/// assert!(errno_name(11).contains("Try again"));
/// ```
pub fn errno_name(errno: i32) -> &'static str {
    match errno {
        1 => "EPERM (Operation not permitted)",
        2 => "ENOENT (No such file or directory)",
        3 => "ESRCH (No such process)",
        4 => "EINTR (Interrupted system call)",
        5 => "EIO (I/O error)",
        6 => "ENXIO (No such device or address)",
        7 => "E2BIG (Argument list too long)",
        8 => "ENOEXEC (Exec format error)",
        9 => "EBADF (Bad file descriptor)",
        10 => "ECHILD (No child processes)",
        11 => "EAGAIN (Try again)",
        12 => "ENOMEM (Out of memory)",
        13 => "EACCES (Permission denied)",
        14 => "EFAULT (Bad address)",
        17 => "EEXIST (File exists)",
        18 => "EXDEV (Cross-device link)",
        19 => "ENODEV (No such device)",
        20 => "ENOTDIR (Not a directory)",
        21 => "EISDIR (Is a directory)",
        22 => "EINVAL (Invalid argument)",
        23 => "ENFILE (File table overflow)",
        24 => "EMFILE (Too many open files)",
        27 => "EFBIG (File too large)",
        28 => "ENOSPC (No space left on device)",
        29 => "ESPIPE (Illegal seek)",
        30 => "EROFS (Read-only file system)",
        31 => "EMLINK (Too many links)",
        32 => "EPIPE (Broken pipe)",
        38 => "ENOSYS (Function not implemented)",
        _ => "UNKNOWN",
    }
}

/// Returns true if the error is retryable (EAGAIN or EINTR).
///
/// # Example
///
/// ```
/// use micronos_kernel::posix::is_error_retryable;
///
/// assert!(is_error_retryable(11)); // EAGAIN
/// assert!(is_error_retryable(4));  // EINTR
/// ```
pub fn is_error_retryable(errno: i32) -> bool {
    matches!(errno, 11 | 4)
}

/// Returns true if the error is fatal and should not be retried.
pub fn is_error_fatal(errno: i32) -> bool {
    matches!(errno, 1 | 9 | 12 | 13 | 14 | 17 | 21 | 22 | 28 | 30)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errno_name() {
        assert!(errno_name(1).starts_with("EPERM"));
        assert!(errno_name(2).starts_with("ENOENT"));
        assert!(errno_name(38).starts_with("ENOSYS"));
        assert_eq!(errno_name(999), "UNKNOWN");
    }

    #[test]
    fn test_is_error_retryable() {
        assert!(is_error_retryable(11)); // EAGAIN
        assert!(is_error_retryable(4)); // EINTR
        assert!(!is_error_retryable(1)); // EPERM
        assert!(!is_error_retryable(9)); // EBADF
    }

    #[test]
    fn test_is_error_fatal() {
        assert!(is_error_fatal(1)); // EPERM
        assert!(is_error_fatal(9)); // EBADF
        assert!(is_error_fatal(12)); // ENOMEM
        assert!(!is_error_fatal(11)); // EAGAIN
        assert!(!is_error_fatal(4)); // EINTR
    }

    #[test]
    fn test_file_descriptor() {
        let fd = FileDescriptor::new(3);
        assert!(fd.is_some());
        let fd = fd.unwrap();
        assert_eq!(fd.fd(), 3);
        assert!(fd.is_valid());

        let invalid = FileDescriptor::new(-1);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_process_id() {
        let pid = ProcessId::new(1234);
        assert_eq!(pid.value(), 1234);

        let default_pid = ProcessId::default();
        assert_eq!(default_pid.value(), 0);
    }

    #[test]
    fn test_thread_id() {
        let tid = ThreadId::new(5678);
        assert_eq!(tid.value(), 5678);

        let default_tid = ThreadId::default();
        assert_eq!(default_tid.value(), 0);
    }
}
