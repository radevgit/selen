//! Propagator management utilities and algorithms

use std::collections::VecDeque;
use super::core_framework::{Propagators, PropId};

// Propagator Management Framework
// 
// This module provides management utilities for organizing constraint propagators.
// It defines queue-based scheduling and propagator lifecycle management.
// 
// ## Framework Overview
// 
// The management framework provides:
// - `PropagatorQueue`: Queue-based scheduling for constraint propagation
// - Lifecycle management for propagator instances
// - Scheduling algorithms for efficient propagation order
// 
// ## Implementation Note
// 
// This is currently a framework-only module to demonstrate the modular architecture
// design. In the final modularization phase, this would provide actual propagator
// scheduling and management functionality.

/// Queue-based propagator scheduling algorithm.
/// 
/// This structure demonstrates how the modular system would schedule
/// and manage constraint propagator execution in the final implementation.
pub struct PropagatorQueue {
    /// Queue of propagators to process.
    pub queue: VecDeque<PropId>,
    
    /// Active propagators in the system.
    pub propagators: Propagators,
}

impl PropagatorQueue {
    /// Create a new propagator queue.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            propagators: Propagators::new(),
        }
    }
    
    /// Add propagator to queue for processing.
    pub fn enqueue(&mut self, prop_id: PropId) {
        self.queue.push_back(prop_id);
    }
    
    /// Get next propagator to process.
    pub fn dequeue(&mut self) -> Option<PropId> {
        self.queue.pop_front()
    }
    
    /// Check if queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    
    /// Get queue length.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
