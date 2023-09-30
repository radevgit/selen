#![allow(dead_code)]

use std::fmt::Display;

use crate::{
    constraint::ConId,
    observer::{MutObserver, Notify, Subject},
    var::VarId,
};

#[derive(Debug)]
pub enum PropagateEvent {
    VarAdd(VarId),                   // Variables is added to solver
    SubOnFix(VarId, ConId),          // Subscribe constraint to variable on_fix event
    SubOnDomainChange(VarId, ConId), // Subscribe constraint to variable on_domain_change event
    SubsOnBoundChange(VarId, ConId), // Subscribe constraint to variable on_bound_change event
    OnFix(VarId),                    // propagate on fix
    OnDomainChange(VarId),           // propagate on domain change
    OnBoundChange(VarId),            // propagate on bound change
    Inconsistency,                   // domain inconsistency
}

impl Display for PropagateEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match self {
        //     PropagateEvent::VarAdd(t) => writeln!(f, "VarAdd({:?})", t),
        //     PropagateEvent::SubOnFix(v, c) => writeln!(f, "SubOnFix({}, {})", v, c),
        //     PropagateEvent::SubOnDomainChange(v, c) => {
        //         writeln!(f, "SubOnDomainChange({}, {})", v, c)
        //     }
        //     PropagateEvent::SubsOnBoundChange(v, c) => {
        //         writeln!(f, "SubOnBoundChange({}, {})", v, c)
        //     }
        //     PropagateEvent::OnFix(_) => todo!(),
        //     PropagateEvent::OnDomainChange(_) => todo!(),
        //     PropagateEvent::OnBoundChange(_) => todo!(),
        //     PropagateEvent::Inconsistency => todo!(),
        // }
        write!(f, "{:?}", self)
    }
}

// Subject for Observer
pub type SubjectPropagate = Subject<PropagateEvent>;

/// A subscriber (listener) has type of a callable function.
//pub type Subscriber = for<'a> fn(&'a mut EventStack, usize, usize, Event);
//pub type Subscriber = for<'a> fn(&'a mut Scheduler, VarId, PropagateEvent);

// Keeps sunscriptions and events and schedules them.
// Scheduler
#[derive(Debug)]
pub struct Scheduler {
    pub(crate) subs_on_fix: Vec<Vec<ConId>>,
    pub(crate) subs_on_domain: Vec<Vec<ConId>>,
    pub(crate) subs_on_bound: Vec<Vec<ConId>>,
    stack: Vec<ConId>,
}

impl MutObserver<PropagateEvent> for Scheduler {
    fn on_notify(&mut self, event: &PropagateEvent) -> Notify {
        match event {
            PropagateEvent::VarAdd(var) => {
                self.var_add(*var);
            }
            PropagateEvent::SubOnFix(v, c) => self.subs_on_fix(*v, *c),
            PropagateEvent::SubOnDomainChange(v, c) => self.subs_on_domain(*v, *c),
            PropagateEvent::SubsOnBoundChange(v, c) => self.subs_on_bound(*v, *c),
            PropagateEvent::OnFix(_) => {
                // TODO
                println!("OnFix()");
            }
            PropagateEvent::OnDomainChange(_) => {
                // TODO
                println!("OnDomainChange()");
            }
            PropagateEvent::OnBoundChange(_) => {
                // TODO
                println!("OnBoundChange()");
            }
            PropagateEvent::Inconsistency => {
                // TODO
                println!("Inconsistency");
            }
        }
        Notify::None
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            subs_on_fix: Vec::new(),
            subs_on_domain: Vec::new(),
            subs_on_bound: Vec::new(),
            stack: Vec::new(),
        }
    }

    // create new subscribtion entry for this new var
    // pub fn subs_add_var(&mut self, var: VarId) {
    //     // prevent double subscription
    //     debug_assert_eq!(var, self.subs_on_domain.len());
    //     self.subs_on_domain.push(Vec::new());
    //     self.subs_on_bound.push(Vec::new());
    // }

    pub fn var_add(&mut self, var: VarId) {
        println!("var_add: {}", var);
        debug_assert_eq!(var, self.subs_on_domain.len());
        // Create subscription arrays
        self.subs_on_fix.push(Vec::new());
        self.subs_on_domain.push(Vec::new());
        self.subs_on_bound.push(Vec::new());
    }
    // Subscribe constraint for variable on_fix
    pub fn subs_on_fix(&mut self, var: VarId, con: ConId) {
        println!("subs_on_fix: {} {}", var, con);
        self.subs_on_fix[var].push(con);
    }

    // Subscribe constraint for variable on_domain
    pub fn subs_on_domain(&mut self, var: VarId, con: ConId) {
        println!("subs_on_domain: {} {}", var, con);
        self.subs_on_domain[var].push(con);
    }

    // Subscribe constraint for variable on_bound
    pub fn subs_on_bound(&mut self, var: VarId, con: ConId) {
        println!("subs_on_bound: {} {}", var, con);
        self.subs_on_bound[var].push(con);
    }

    pub fn notify(&mut self, var: VarId, event: &PropagateEvent) {
        print!("var: {} event:{}", var, event);
    }

    pub fn subs_on_fix_size(&self) -> usize {
        self.subs_on_fix.len()
    }

    pub fn subs_on_domain_size(&self) -> usize {
        self.subs_on_domain.len()
    }

    pub fn subs_on_bound_size(&self) -> usize {
        self.subs_on_bound.len()
    }

    pub fn subs_var_con_size(&self, v: VarId) -> (usize, usize, usize) {
        let on_fix = self.subs_on_fix[v].len();
        let on_domain = self.subs_on_domain[v].len();
        let on_bound = self.subs_on_bound[v].len();
        (on_fix, on_domain, on_bound)
    }

    pub fn pop(&mut self) -> Option<VarId> {
        self.stack.pop()
    }

    pub fn clear(&mut self) {
        self.stack.clear()
    }
}

#[cfg(test)]
mod test_scheduler {
    use std::{cell::RefCell, ops::Deref, rc::Rc};

    use crate::{
        solver::Solver, solver_options::SolverOptions, state::StateEvent, state_int::StateInt, var::Var,
    };

    use super::*;

    #[test]
    fn test_debug_event() {
        assert_eq!(
            format!("{:?}", PropagateEvent::SubsOnBoundChange(5, 1)),
            "SubsOnBoundChange(5, 1)"
        );
    }

    #[test]
    fn test_display_event() {
        assert_eq!(format!("{}", PropagateEvent::VarAdd(5)), "VarAdd(5)");
    }

    #[test]
    fn test_debug_scheduler() {
        let o = Rc::new(RefCell::new(Scheduler::new()));
        assert_eq!(format!("{:?}", o), "RefCell { value: Scheduler { subs_on_fix: [], subs_on_domain: [], subs_on_bound: [], stack: [] } }");
    }

    #[test]
    fn test_subject() {
        let s = StateInt {
            id: 5,
            observer: Subject::new(),
        };
        let res = s.notify(&StateEvent::IntSet(0, 0));
        assert_eq!(res, Notify::None);
    }

    #[test]
    fn test_notify() {
        let opt = SolverOptions {};
        let mut sol = Solver::new(opt);
        sol.var_int(0, 10);
        let t = (sol.vars[0]).clone();
        let tt = t.borrow();
        if let Var::VarI(tt) = tt.deref() {
            tt.notify(&PropagateEvent::SubsOnBoundChange(0, 3));
        }
    }
}
