//! Custom allocator for tracking memory usage
//!
//! This module provides a `TrackingAllocator` that wraps the system allocator
//! and keeps track of total allocated bytes. This is useful for profiling
//! and understanding the memory footprint of different algorithms.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A wrapper around the system allocator that tracks memory usage
pub struct TrackingAllocator {
    allocator: System,
    allocated_bytes: AtomicUsize,
    allocation_count: AtomicUsize,
}

impl Default for TrackingAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl TrackingAllocator {
    pub const fn new() -> Self {
        Self {
            allocator: System,
            allocated_bytes: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
        }
    }

    /// Reset the statistics
    pub fn reset(&self) {
        self.allocated_bytes.store(0, Ordering::SeqCst);
        self.allocation_count.store(0, Ordering::SeqCst);
    }

    /// Get total bytes currently allocated (net)
    /// Note: This simple implementation only tracks growing usage loosely
    /// as deallocations might not exactly match allocations in complex scenarios,
    /// but for benchmarking specific runs it gives a good delta.
    ///
    /// Actually, for a precise "currently allocated", we subtract on dealloc.
    /// For "total traffic", we might want another counter.
    /// Let's track:
    /// 1. Current usage (alloc - dealloc)
    pub fn get_current_usage(&self) -> usize {
        self.allocated_bytes.load(Ordering::SeqCst)
    }

    pub fn get_allocation_count(&self) -> usize {
        self.allocation_count.load(Ordering::SeqCst)
    }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.allocator.alloc(layout);
        if !ptr.is_null() {
            self.allocated_bytes
                .fetch_add(layout.size(), Ordering::SeqCst);
            self.allocation_count.fetch_add(1, Ordering::SeqCst);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.allocator.dealloc(ptr, layout);
        self.allocated_bytes
            .fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // We can't easily test GlobalAlloc integration directly in unit tests
    // because the global allocator is set at compile time for the binary.
    // However, we can test the struct logic if we could call alloc/dealloc manually,
    // but those are unsafe traits.
    // Instead, we can verify the state management methods.

    #[test]
    fn test_allocator_initial_state() {
        let allocator = TrackingAllocator::new();
        assert_eq!(allocator.get_current_usage(), 0);
        assert_eq!(allocator.get_allocation_count(), 0);
    }

    #[test]
    fn test_manual_tracking_simulation() {
        let allocator = TrackingAllocator::new();
        let layout = Layout::from_size_align(1024, 8).unwrap();

        // Manually simulating alloc
        unsafe {
            let ptr = allocator.alloc(layout);
            assert!(!ptr.is_null());
            assert_eq!(allocator.get_current_usage(), 1024);
            assert_eq!(allocator.get_allocation_count(), 1);

            // Simulating dealloc
            allocator.dealloc(ptr, layout);
            assert_eq!(allocator.get_current_usage(), 0);
            assert_eq!(allocator.get_allocation_count(), 1); // count keeps increasing
        }
    }

    #[test]
    fn test_reset() {
        let allocator = TrackingAllocator::new();
        let layout = Layout::from_size_align(100, 4).unwrap();

        unsafe {
            let ptr = allocator.alloc(layout);
            assert_eq!(allocator.get_current_usage(), 100);

            allocator.reset();
            assert_eq!(allocator.get_current_usage(), 0);
            assert_eq!(allocator.get_allocation_count(), 0);

            // Cleanup
            allocator.dealloc(ptr, layout);
            // Usage will underflow or go negative if we rely on usize simple subtraction,
            // but since it's atomic usize wrapping, it will be large.
            // This is just a test artifact; in real usage we wouldn't reset while memory is alive
            // unless we wanted to start fresh counting from that point.
        }
    }
}
