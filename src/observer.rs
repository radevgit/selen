#![allow(dead_code)]

// https://github.com/kan1-u/event-observer/blob/master/src/observer.rs

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum Notify {
    None,
    Int(u32),
    Float(f64),
}

pub trait Observer<E> {
    //fn on_notify(&self, event: &E);
    fn on_notify(&self, event: &E) -> Notify;
}

pub trait MutObserver<E> {
    //fn on_notify(&mut self, event: &E);
    fn on_notify(&mut self, event: &E) -> Notify;
}

// Not needed
// impl<E> Observer<E> for Rc<RefCell<dyn Observer<E>>> {
//     // fn on_notify(&self, event: &E) {
//     //     self.borrow().on_notify(event)
//     // }
//     fn on_notify(&self, event: &E) -> Notify {
//         self.borrow().on_notify(event)
//     }
// }

impl<E> Observer<E> for Rc<RefCell<dyn MutObserver<E>>> {
    // fn on_notify(&self, event: &E) {
    //     self.borrow_mut().on_notify(event)
    // }
    fn on_notify(&self, event: &E) -> Notify {
        self.borrow_mut().on_notify(event)
    }
}

// https://github.com/kan1-u/event-observer/blob/master/src/subject.rs
pub struct Subject<E> {
    observer: Option<Box<dyn Observer<E>>>,
}

impl<E> Subject<E> {
    pub fn new() -> Self {
        Self { observer: None }
    }

    // pub fn notify(&self, event: &E) {
    //     if let Some(ob) = self.observer {
    //         ob.on_notify(event)
    //     }
    // }

    pub fn notify(&self, event: &E) -> Notify {
        let o = &(*self).observer;
        if let Some(ob) = o {
            ob.on_notify(event)
        } else {
            Notify::None
        }
    }

    pub fn add_observer(&mut self, observer: impl Observer<E> + 'static) {
        self.observer = Some(Box::new(observer));
    }

    // pub fn remove_observer(&mut self) {
    //     self.observer = None;
    // }
}

impl<E: 'static> Subject<E> {
    // Not needed
    // pub fn add_rc_refcell_observer(&mut self, observer: Rc<RefCell<dyn Observer<E>>>) {
    //     self.add_observer(observer)
    // }

    pub fn add_rc_refcell_mut_observer(&mut self, observer: Rc<RefCell<dyn MutObserver<E>>>) {
        self.add_observer(observer)
    }
}

#[cfg(test)]
mod vint_tests {
    use crate::{
        observer::Notify,
        scheduler::{PropagateEvent, SubjectPropagate},
        state::{StateEvent, SubjectState},
    };

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Notify::None), "None");
    }

    #[test]
    fn test_subject_state() {
        let s = SubjectState::new(); // Not initialized observer
        let res = s.notify(&StateEvent::IntSet(0, 0));
        assert_eq!(res, Notify::None);
    }

    #[test]
    fn test_subject_propagate() {
        let s = SubjectPropagate::new(); // Not initialized observer
        let res = s.notify(&PropagateEvent::Inconsistency);
        assert_eq!(res, Notify::None);
    }
}
