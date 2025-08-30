use std::collections::{HashSet, VecDeque};

use crate::props::PropId;

/// Collection of propagators scheduled to be run.
#[derive(Debug, Default)]
pub struct Agenda {
    q: VecDeque<PropId>,
    h: HashSet<PropId>,
}

impl Agenda {
    /// Initialize agenda and schedule the provided propagators.
    pub fn with_props(ps: impl Iterator<Item = PropId>) -> Self {
        let mut agenda = Self::default();

        for p in ps {
            agenda.schedule(p);
        }

        agenda
    }

    /// Schedule a propagator if it is not already on the agenda.
    pub fn schedule(&mut self, p: PropId) {
        // Avoid scheduling a propagator already on the agenda
        if !self.h.contains(&p) {
            // Schedule propagators in FIFO order to avoid starvation
            self.q.push_back(p);

            // Scheduled propagators are also stored in a hash set to allow fast look-up
            let _was_in_hashet = self.h.insert(p);
        }
    }

    /// Acquire handle to next propagator to run, removing it from the [`Agenda`].
    pub fn pop(&mut self) -> Option<PropId> {
        // Pop scheduled propagators in FIFO order to avoid starvation
        let p = self.q.pop_front()?;

        // Scheduled propagators are also stored in a hash set to allow fast look-up
        let _was_in_hashet = self.h.remove(&p);

        Some(p)
    }
}