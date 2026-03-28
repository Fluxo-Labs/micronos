use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    Free,
    Code,
    Data,
    Heap,
    Stack,
    MMIO,
}

#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: usize,
    pub size: usize,
    pub region_type: MemoryRegionType,
    pub name: String,
}

impl MemoryRegion {
    pub fn new(start: usize, size: usize, region_type: MemoryRegionType, name: &str) -> Self {
        MemoryRegion {
            start,
            size,
            region_type,
            name: name.to_string(),
        }
    }

    pub fn end(&self) -> usize {
        self.start + self.size
    }

    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.start && addr < self.end()
    }

    pub fn overlaps(&self, other: &MemoryRegion) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub heap_used: u64,
    pub heap_allocations: u32,
    pub stack_size: u64,
    pub regions_count: usize,
}

pub struct MemoryManager {
    total_memory: u64,
    regions: Vec<MemoryRegion>,
    heap_allocations: u32,
    heap_used: u64,
    stack_size: u64,
}

impl MemoryManager {
    pub fn new(total_bytes: u64) -> Self {
        let mut mm = MemoryManager {
            total_memory: total_bytes,
            regions: Vec::new(),
            heap_allocations: 0,
            heap_used: 0,
            stack_size: 0,
        };
        mm.initialize_regions();
        mm
    }

    fn initialize_regions(&mut self) {
        let code_size = 64 * 1024;
        let data_size = 32 * 1024;
        let stack_size = 8 * 1024 * 1024;

        let mut addr = 0x1000;

        self.regions.push(MemoryRegion::new(
            addr,
            code_size,
            MemoryRegionType::Code,
            "kernel_code",
        ));
        addr += code_size;

        self.regions.push(MemoryRegion::new(
            addr,
            data_size,
            MemoryRegionType::Data,
            "kernel_data",
        ));
        addr += data_size;

        let heap_start = addr;
        let reserved = heap_start + stack_size;
        let heap_size = (self.total_memory as usize).saturating_sub(reserved);
        self.regions.push(MemoryRegion::new(
            heap_start,
            heap_size,
            MemoryRegionType::Heap,
            "main_heap",
        ));
        addr += heap_size;

        self.regions.push(MemoryRegion::new(
            addr,
            stack_size,
            MemoryRegionType::Stack,
            "kernel_stack",
        ));

        self.heap_used = 0;
        self.heap_allocations = 0;
        self.stack_size = stack_size as u64;
    }

    pub fn allocate(&mut self, size: u64) -> Option<usize> {
        if self.heap_used + size > self.total_memory / 2 {
            return None;
        }

        let base = 0x10000 + self.heap_used as usize;
        self.heap_used += size;
        self.heap_allocations += 1;

        Some(base)
    }

    pub fn deallocate(&mut self, addr: usize, size: u64) -> bool {
        if addr < 0x10000 {
            return false;
        }

        self.heap_used = self.heap_used.saturating_sub(size);
        self.heap_allocations = self.heap_allocations.saturating_sub(1);
        true
    }

    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            total: self.total_memory,
            used: self.heap_used,
            free: self.total_memory - self.heap_used,
            heap_used: self.heap_used,
            heap_allocations: self.heap_allocations,
            stack_size: self.stack_size,
            regions_count: self.regions.len(),
        }
    }

    pub fn get_regions(&self) -> &[MemoryRegion] {
        &self.regions
    }

    pub fn find_region(&self, addr: usize) -> Option<&MemoryRegion> {
        self.regions.iter().find(|r| r.contains(addr))
    }

    pub fn total_memory(&self) -> u64 {
        self.total_memory
    }

    pub fn used_memory(&self) -> u64 {
        self.heap_used
    }

    pub fn free_memory(&self) -> u64 {
        self.total_memory - self.heap_used
    }

    pub fn usage_percent(&self) -> f64 {
        if self.total_memory == 0 {
            0.0
        } else {
            (self.heap_used as f64 / self.total_memory as f64) * 100.0
        }
    }

    pub fn format_stats(&self) -> String {
        let stats = self.get_stats();
        let used_mb = stats.used as f64 / (1024.0 * 1024.0);
        let total_mb = stats.total as f64 / (1024.0 * 1024.0);
        let free_mb = stats.free as f64 / (1024.0 * 1024.0);
        let heap_mb = stats.heap_used as f64 / (1024.0 * 1024.0);
        let stack_mb = stats.stack_size as f64 / (1024.0 * 1024.0);

        let mut output =
            String::from("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                    Memory Manager                           ║\n");
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");
        output.push_str(&alloc::format!(
            "║  Total Memory:   {:>10.2} MB ({:>15} bytes)       ║\n",
            total_mb,
            stats.total
        ));
        output.push_str(&alloc::format!(
            "║  Used Memory:    {:>10.2} MB ({:>15} bytes)       ║\n",
            used_mb,
            stats.used
        ));
        output.push_str(&alloc::format!(
            "║  Free Memory:    {:>10.2} MB ({:>15} bytes)       ║\n",
            free_mb,
            stats.free
        ));
        output.push_str(&alloc::format!(
            "║  Usage:          {:>6.1}%                                    ║\n",
            self.usage_percent()
        ));
        output.push_str("╠════════════════════════════════════════════════════════════════╣\n");
        output.push_str("║  Heap:                                                        ║\n");
        output.push_str(&alloc::format!(
            "║    Used:           {:>10.2} MB                         ║\n",
            heap_mb
        ));
        output.push_str(&alloc::format!(
            "║    Allocations:    {:>10}                                 ║\n",
            stats.heap_allocations
        ));
        output.push_str("║  Stack:                                                       ║\n");
        output.push_str(&alloc::format!(
            "║    Size:            {:>10.2} MB                         ║\n",
            stack_mb
        ));
        output.push_str(&alloc::format!(
            "║  Regions:          {:>10}                                 ║\n",
            stats.regions_count
        ));
        output.push_str("╚════════════════════════════════════════════════════════════════╝");
        output
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new(1_073_741_824)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_manager() {
        let mm = MemoryManager::new(1024 * 1024);
        assert_eq!(mm.total_memory(), 1024 * 1024);
    }

    #[test]
    fn test_allocate() {
        let mut mm = MemoryManager::new(1024 * 1024);
        let addr = mm.allocate(100);
        assert!(addr.is_some());
    }

    #[test]
    fn test_deallocate() {
        let mut mm = MemoryManager::new(1024 * 1024);
        let addr = mm.allocate(100).unwrap();
        assert!(mm.deallocate(addr, 100));
    }

    #[test]
    fn test_stats() {
        let mut mm = MemoryManager::new(1024 * 1024);
        mm.allocate(100);
        let stats = mm.get_stats();
        assert!(stats.heap_allocations >= 1);
    }
}
