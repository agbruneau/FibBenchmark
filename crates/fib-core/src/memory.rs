//! Memory profiling utilities

use crate::allocator::TrackingAllocator;

/// Snapshot of memory statistics
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub current_bytes: usize,
    pub allocations: usize,
}

impl MemoryStats {
    pub fn now(allocator: &TrackingAllocator) -> Self {
        Self {
            current_bytes: allocator.get_current_usage(),
            allocations: allocator.get_allocation_count(),
        }
    }

    /// Calculate delta from a previous snapshot
    pub fn delta(&self, start: &MemoryStats) -> Self {
        Self {
            current_bytes: self.current_bytes.saturating_sub(start.current_bytes),
            allocations: self.allocations.saturating_sub(start.allocations),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_stats_delta() {
        let start = MemoryStats {
            current_bytes: 100,
            allocations: 5,
        };

        let end = MemoryStats {
            current_bytes: 250,
            allocations: 8,
        };

        let delta = end.delta(&start);
        assert_eq!(delta.current_bytes, 150);
        assert_eq!(delta.allocations, 3);
    }

    #[test]
    fn test_memory_stats_delta_saturating() {
        // Case where memory usage decreased (freed)
        let start = MemoryStats {
            current_bytes: 500,
            allocations: 10,
        };

        let end = MemoryStats {
            current_bytes: 200, // less usage
            allocations: 12,    // more allocs count
        };

        let delta = end.delta(&start);
        assert_eq!(delta.current_bytes, 0); // saturating_sub
        assert_eq!(delta.allocations, 2);
    }
}
