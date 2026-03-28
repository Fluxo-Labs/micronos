use micronos_fs::storage::StorageManager;
use micronos_fs::storage::StorageState;
use micronos_fs::vfs::VFSState;
use micronos_fs::vfs::VirtualFileSystem;

#[test]
fn test_vfs_mount_workflow() {
    let mut vfs = VirtualFileSystem::new();
    vfs.mount();
    assert!(matches!(vfs.state, VFSState::Mounting));

    vfs.complete_mount();
    assert!(matches!(vfs.state, VFSState::Ready));
}

#[test]
fn test_vfs_readonly() {
    let mut vfs = VirtualFileSystem::new();
    vfs.mount();
    vfs.complete_mount();
    vfs.make_readonly();
    assert!(matches!(vfs.state, VFSState::ReadOnly));
}

#[test]
fn test_vfs_root_entries() {
    let vfs = VirtualFileSystem::new();
    let entries = vfs.root_entries();
    assert!(!entries.is_empty());
}

#[test]
fn test_storage_lifecycle() {
    let mut storage = StorageManager::new(1_000_000);
    assert!(matches!(storage.state, StorageState::Offline));

    storage.initialize();
    assert!(matches!(storage.state, StorageState::Initializing));

    storage.go_online();
    assert!(matches!(storage.state, StorageState::Online));

    storage.suspend();
    assert!(matches!(storage.state, StorageState::Suspended));

    storage.resume();
    assert!(matches!(storage.state, StorageState::Online));
}

#[test]
fn test_storage_availability() {
    let storage = StorageManager::new(1_000_000);
    assert_eq!(storage.available(), 1_000_000);
}
