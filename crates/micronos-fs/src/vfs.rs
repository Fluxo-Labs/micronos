use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use micronos_core::traits::DirEntry;

#[derive(Debug, Clone, Default)]
pub enum VFSState {
    #[default]
    Unmounted,
    Mounting,
    Ready,
    ReadOnly,
    Error,
}

pub struct VirtualFileSystem {
    pub state: VFSState,
    #[allow(dead_code)]
    pub mount_points: Vec<MountPoint>,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        VirtualFileSystem {
            state: VFSState::Unmounted,
            mount_points: Vec::new(),
        }
    }

    pub fn mount(&mut self) {
        if matches!(self.state, VFSState::Unmounted) {
            self.state = VFSState::Mounting;
        }
    }

    pub fn complete_mount(&mut self) {
        if matches!(self.state, VFSState::Mounting) {
            self.state = VFSState::Ready;
        }
    }

    pub fn make_readonly(&mut self) {
        if matches!(self.state, VFSState::Ready) {
            self.state = VFSState::ReadOnly;
        }
    }

    pub fn unmount(&mut self) {
        self.state = VFSState::Unmounted;
    }

    pub fn root_entries(&self) -> Vec<DirEntry> {
        vec![
            DirEntry::new("bin", true, 0),
            DirEntry::new("etc", true, 0),
            DirEntry::new("home", true, 0),
            DirEntry::new("var", true, 0),
            DirEntry::new("usr", true, 0),
        ]
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, VFSState::Ready)
    }
}

impl Default for VirtualFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct MountPoint {
    pub path: String,
    pub device: String,
    pub fs_type: String,
}

#[derive(Debug, Clone, Default)]
pub enum FileState {
    #[default]
    Closed,
    Open,
}

pub struct MemoryFileHandle {
    state: FileState,
    #[allow(dead_code)]
    data: Vec<u8>,
    position: usize,
}

impl MemoryFileHandle {
    pub fn new() -> Self {
        MemoryFileHandle {
            state: FileState::Open,
            data: Vec::new(),
            position: 0,
        }
    }

    pub fn close_file(&mut self) {
        self.state = FileState::Closed;
    }

    pub fn reopen(&mut self) {
        self.state = FileState::Open;
        self.position = 0;
    }

    pub fn is_open(&self) -> bool {
        matches!(self.state, FileState::Open)
    }
}

impl Default for MemoryFileHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_creation() {
        let vfs = VirtualFileSystem::new();
        assert!(matches!(vfs.state, VFSState::Unmounted));
    }

    #[test]
    fn test_mount_workflow() {
        let mut vfs = VirtualFileSystem::new();
        vfs.mount();
        assert!(matches!(vfs.state, VFSState::Mounting));
        vfs.complete_mount();
        assert!(matches!(vfs.state, VFSState::Ready));
    }

    #[test]
    fn test_readonly_transition() {
        let mut vfs = VirtualFileSystem::new();
        vfs.mount();
        vfs.complete_mount();
        vfs.make_readonly();
        assert!(matches!(vfs.state, VFSState::ReadOnly));
    }

    #[test]
    fn test_file_handle() {
        let mut handle = MemoryFileHandle::new();
        assert!(handle.is_open());
        handle.close_file();
        assert!(!handle.is_open());
    }

    #[test]
    fn test_root_entries() {
        let vfs = VirtualFileSystem::new();
        let entries = vfs.root_entries();
        assert!(!entries.is_empty());
        assert!(entries.iter().all(|e| e.is_dir));
    }
}
