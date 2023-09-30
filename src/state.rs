#![allow(dead_code)]

use std::fmt::Display;

use crate::observer::{MutObserver, Notify, Subject};

pub type StateId = usize;

pub type SubjectState = Subject<StateEvent>;

#[derive(Debug, PartialEq)]
pub enum StateEvent {
    MakeStateInt(u32),
    IntSet(StateId, u32),
    IntValue(StateId),
}

impl Display for StateEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateEvent::MakeStateInt(val) => write!(f, "MakeStateInt({})", val),
            StateEvent::IntSet(id, val) => write!(f, "IntSet({}, {})", id, val),
            StateEvent::IntValue(id) => write!(f, "IntValue({})", id),
        }
    }
}

// Variable history store
#[derive(Debug)]
pub struct State {
    pub int_states: Vec<u32>,
}

impl MutObserver<StateEvent> for State {
    fn on_notify(&mut self, event: &StateEvent) -> Notify {
        match event {
            StateEvent::MakeStateInt(val) => self.make_state_int(*val),
            StateEvent::IntSet(id, val) => {
                self.on_int_set(*id,* val);
                Notify::None
            }
            StateEvent::IntValue(id) => self.on_int_value(*id),
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            int_states: Vec::new(),
        }
    }
    // Adds new state int, returns state int id
    fn make_state_int(&mut self, val: u32) -> Notify {
        // TODO: prevent adding twice
        println!("State:MakeStateInt()");
        let id = self.int_states.len();
        self.int_states.push(val);
        Notify::Int(id as u32)
    }

    fn on_int_set(&mut self, id: StateId, val: u32) {
        self.int_states[id] = val;
    }

    fn on_int_value(&self, id: StateId) -> Notify {
        return Notify::Int(self.int_states[id]);
    }
}

#[cfg(test)]
mod test_state {
    use std::{cell::RefCell, rc::Rc};

    use crate::state_int::StateInt;

    use super::*;

    #[test]
    fn test_display_event() {
        assert_eq!(format!("{}", StateEvent::IntSet(1, 2)), "IntSet(1, 2)");
        assert_eq!(format!("{}", StateEvent::IntValue(1)), "IntValue(1)");
        assert_eq!(format!("{}", StateEvent::MakeStateInt(3)), "MakeStateInt(3)");
    }

    #[test]
    fn test_debug_event() {
        assert_eq!(format!("{:?}", StateEvent::IntSet(1, 2)), "IntSet(1, 2)");
        assert_eq!(format!("{:?}", StateEvent::IntValue(1)), "IntValue(1)");
        assert_eq!(format!("{:?}", StateEvent::MakeStateInt(3)), "MakeStateInt(3)");
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
