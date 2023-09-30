#![allow(dead_code)]

use std::fmt::{self, Debug};
use std::{cell::RefCell, rc::Rc};

use crate::{
    observer::{MutObserver, Notify},
    state::{StateEvent, StateId, SubjectState},
};

pub struct StateInt {
    pub id: StateId,
    pub observer: SubjectState,
}

impl Debug for StateInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl StateInt {
    pub fn new(val: u32, state: Rc<RefCell<dyn MutObserver<StateEvent>>>) -> Self {
        let mut s = Self {
            id: 0,
            observer: SubjectState::new(),
        };
        s.observer.add_rc_refcell_mut_observer(state);
        // Subsrcibe to state manager
        let ret = s.notify(&StateEvent::MakeStateInt(val));
        if let Notify::Int(id) = ret {
            s.id = id as usize;
        }
        s
    }

    pub fn notify(&self, event: &StateEvent) -> Notify {
        self.observer.notify(event)
    }

    pub fn set_value(&self, val: u32) {
        let _ = self.notify(&StateEvent::IntSet(self.id, val));
    }

    pub fn value(&self) -> u32 {
        match self.notify(&StateEvent::IntValue(self.id)) {
            Notify::Int(val) => return val,
            _ => u32::MAX, // expected only int value
        }
    }
}

#[cfg(test)]
mod test_state_int {
    use crate::state::State;

    use super::*;

    #[test]
    fn test_debug() {
        let state = Rc::new(RefCell::new(State::new()));
        let st = StateInt::new(0, state);
        st.set_value(55);
        assert_eq!(format!("{:?}", st), "55");
    }

    #[test]
    fn test_value() {
        let state = Rc::new(RefCell::new(State::new()));
        let st = StateInt::new(0, state);
        st.set_value(44);
        st.set_value(55);
        assert_eq!(st.value(), 55);
    }
}
