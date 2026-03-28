use crate::error::Result;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub trait Scheduler: Send + Sync {
    fn schedule(&self) -> Option<ThreadId>;
    fn add_thread(&self, thread: ThreadId, priority: Priority) -> Result<()>;
    fn remove_thread(&self, thread: ThreadId) -> Result<()>;
    fn yield_current(&self) -> Result<()>;
    fn set_priority(&self, thread: ThreadId, priority: Priority) -> Result<()>;
}

pub trait MemoryManager: Send + Sync {
    fn allocate(&self, size: MemorySize) -> Result<MemoryAddress>;
    fn deallocate(&self, addr: MemoryAddress, size: MemorySize) -> Result<()>;
    fn translate(&self, virt: MemoryAddress) -> Option<MemoryAddress>;
}

pub trait NetworkStack: Send + Sync {
    fn send(&self, data: &[u8], to: NodeId) -> Result<()>;
    fn receive(&self, timeout_ms: u64) -> Result<(Vec<u8>, NodeId)>;
    fn connected_nodes(&self) -> Vec<NodeId>;
}

pub trait FileSystem: Send + Sync {
    fn open(&self, path: &str) -> Result<FileHandle>;
    fn create(&self, path: &str) -> Result<FileHandle>;
    fn remove(&self, path: &str) -> Result<()>;
    fn list(&self, path: &str) -> Result<Vec<DirEntry>>;
}

#[derive(Default)]
pub struct FileHandle;

impl FileHandle {
    pub fn new() -> Self {
        FileHandle
    }

    pub fn read(&self, _buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }

    pub fn write(&self, _buf: &[u8]) -> Result<usize> {
        Ok(0)
    }

    pub fn seek(&self, _pos: u64) -> Result<()> {
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
}

impl DirEntry {
    pub fn new(name: &str, is_dir: bool, size: u64) -> Self {
        DirEntry {
            name: name.to_string(),
            is_dir,
            size,
        }
    }
}

pub use crate::types::{MemoryAddress, MemorySize, NodeId, Priority, ProcessId, ThreadId};
